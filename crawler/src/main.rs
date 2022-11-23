#[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;
// static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    crawler::archive::crawl_files("workbench/data/pgcr");
}
