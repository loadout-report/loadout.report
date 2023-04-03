use std::fs::create_dir_all;

const OUT_DIR: &str = "./src/generated";

fn main() {

    if std::path::Path::new(OUT_DIR).exists() {
        std::fs::remove_dir_all(OUT_DIR).unwrap();
    }

    create_dir_all(OUT_DIR).unwrap();

    let spec = openapi_generator::read_spec("openapi.json");
    openapi_generator::generate(spec, OUT_DIR);
}
