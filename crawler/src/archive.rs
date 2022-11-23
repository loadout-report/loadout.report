use indicatif::{
    HumanDuration, MultiProgress, ParallelProgressIterator, ProgressBar, ProgressIterator,
    ProgressState, ProgressStyle,
};

use model::ArchivedReport;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

use std::collections::HashMap;
use std::fmt::Write;
use std::fs::{read_dir, File};

use std::io::{BufRead, BufReader};

use std::time::{Duration, Instant};

pub mod model;

const CHUNK_SIZE: u64 = 10_000_000;

pub fn crawl_files(dir: &str) {
    let files = read_dir(dir).expect("unable to open directory").count();
    let multi = MultiProgress::new();
    let bar = multi.add(ProgressBar::new(files as u64));
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {percent:>3}% {smoothed_per_sec} {smoothed_eta} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
        .with_key(
        "smoothed_eta",
        |s: &ProgressState, w: &mut dyn Write| match (s.pos(), s.len()) {
            (pos, Some(len)) => write!(
                w,
                "{:#}",
                HumanDuration(Duration::from_millis(
                    (s.elapsed().as_millis() * (len as u128 - pos as u128) / (pos as u128))
                        as u64
                ))
            )
                .unwrap(),
            _ => write!(w, "-").unwrap(),
        },
    )
    .with_key(
        "smoothed_per_sec",
        |s: &ProgressState, w: &mut dyn Write| match (s.pos(), s.elapsed().as_millis()) {
            (pos, elapsed_ms) if elapsed_ms > 0 => {
                write!(w, "{:.2}/s", pos as f64 * 1000_f64 / elapsed_ms as f64).unwrap()
            }
            _ => write!(w, "-").unwrap(),
        },
    )
    .progress_chars("##-");
    bar.set_style(sty.clone());

    let time = Instant::now();
    let map: HashMap<String, u32> = read_dir(dir)
        .expect("unable to open directory")
        .flatten()
        .progress_with(bar)
        .map(|dir_entry| {
            let pb = multi.add(ProgressBar::new(CHUNK_SIZE));
            let file_name = dir_entry.file_name();
            pb.set_message(file_name.to_str().unwrap().to_string()); // end me
            pb.set_style(sty.clone());
            let file = File::open(dir_entry.path()).expect("couldn't open archive file");
            let decoder = zstd::stream::Decoder::new(file).expect("couldn't open decoder");
            let reader = BufReader::new(decoder);

            let parallel_line_reader = reader
                .lines()
                .filter_map(|l| l.ok())
                .par_bridge()
                .progress_with(pb);
            let json_reader = parse_json(parallel_line_reader);
            extract_gms(json_reader)
        })
        .reduce(merge_sum)
        .unwrap();

    let elapsed = time.elapsed();
    let time_per_entry = elapsed / (files * CHUNK_SIZE as usize) as u32;
    println!(
        "elapsed: {:?}, time per entry: {:?}",
        elapsed, time_per_entry,
    );

    let output_file = File::create("out.json.zst").unwrap();
    let writer = zstd::Encoder::new(output_file, 0).unwrap();
    serde_json::to_writer(writer, &map).unwrap();
}

#[inline]
fn parse_json(
    parallel_line_reader: impl ParallelIterator<Item = String>,
) -> impl ParallelIterator<Item = ArchivedReport> {
    parallel_line_reader.filter_map(|l: String|
        // simd_json::from_str::<ArchivedReport>(l.clone().as_mut_str())
        serde_json::from_str(&l)
            .inspect_err(|err| println!("{}, {}", err, l))
            .ok())
}

fn merge_sum(mut a: HashMap<String, u32>, b: HashMap<String, u32>) -> HashMap<String, u32> {
    for (k, v) in b {
        *a.entry(k).or_default() += &v;
    }
    a
}

fn extract_gms(json_reader: impl ParallelIterator<Item = ArchivedReport>) -> HashMap<String, u32> {
    json_reader
        .filter(|pcgr| pcgr.is_gm())
        .fold(
            HashMap::new,
            |mut a: HashMap<String, u32>, b: ArchivedReport| {
                let k = format!("strike/{}/season/{}", b.strike_slug(), b.season());
                *a.entry(format!("{}/a", k)).or_default() += &1;
                if b.is_completed() {
                    *a.entry(format!("{}/c", k)).or_default() += &1;
                }
                for entry in &b.entries {
                    let k = format!(
                        "player/{}/{}/{}",
                        entry.player.destiny_user_info.membership_type,
                        entry.player.destiny_user_info.membership_id.0,
                        k
                    );
                    *a.entry(format!("{}/a", k)).or_default() += &1;
                    if entry.values.completed > 0.5 {
                        *a.entry(format!("{}/c", k)).or_default() += &1;
                    }
                }
                a
            },
        )
        .reduce(HashMap::new, merge_sum)
}
