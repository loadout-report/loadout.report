use jemallocator::Jemalloc;

#[global_allocator]
// static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    crawler::archive::crawl_files("workbench/data/pgcr");
}
