use indicatif::{
    HumanDuration, MultiProgress, ParallelProgressIterator, ProgressBar, ProgressIterator,
    ProgressState, ProgressStyle,
};

use crate::pgcr::archive::ArchivedReport;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;

use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::fs::{read_dir, File};

use std::io::{BufRead, BufReader};

use std::time::{Duration, Instant};
use crate::pgcr::extract::{extract_gms, extract_members, merge, merge_sum};

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
    let map: HashSet<_> = read_dir(dir) // each file in a directory as an iterator
        .expect("unable to open directory")
        .flatten()
        .par_bridge() // parallelise here
        .progress_with(bar) // files started progress bar
        .map(|dir_entry| {
            let file_name = dir_entry.file_name();

            // progress bar stuff
            let pb = multi.add(ProgressBar::new(CHUNK_SIZE));
            pb.set_message(file_name.to_str().unwrap().to_string()); // end me
            pb.set_style(sty.clone());

            let file = File::open(dir_entry.path()).expect("couldn't open archive file");
            let decoder = zstd::stream::Decoder::new(file).expect("couldn't open decoder");
            let reader = BufReader::new(decoder);

            let parallel_line_reader = reader
                .lines()
                .filter_map(|l| l.ok()) // ignore lines that aren't valid utf-8
                .par_bridge() // parallelise again here
                .progress_with(pb); // progress bar
            parse_json(parallel_line_reader) // parse json
        })
        .map(extract_members)
        .reduce(HashSet::new, merge);

    let elapsed = time.elapsed();
    let time_per_entry = elapsed / (files * CHUNK_SIZE as usize) as u32;
    println!(
        "elapsed: {:?}, time per entry: {:?}",
        elapsed, time_per_entry,
    );

    let output_file = File::create("out.json.zst").expect("could not open file for writing");
    let writer =
        zstd::Encoder::new(output_file, 0).expect("could not create zstd encoder for output");
    serde_json::to_writer(writer, &map).expect("could not serialize json output");
}

#[inline]
fn parse_json(
    parallel_line_reader: impl ParallelIterator<Item = String>,
) -> impl ParallelIterator<Item = ArchivedReport> {
    parallel_line_reader.filter_map(|mut l: String|
        unsafe {
            // simd_json::serde::from_slice(l.as_bytes_mut())
                serde_json::from_str(&l)
                .inspect_err(|err| println!("{}, {}", err, l))
                .ok()
        })

}


