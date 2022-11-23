use criterion::criterion_main;
mod archive;

criterion_main! {
    archive::benches,
}
