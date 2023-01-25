use std::env::args;
use std::fs::create_dir_all;

fn main() {
    args().for_each(|arg| println!("{}", arg));
    if args().len() < 3 {
        println!("Usage: openapi-generator <input> <output>");
        return;
    }

    let input = args().nth(1).unwrap();
    let output = args().nth(2).unwrap();

    // output most not exist
    if std::path::Path::new(&output).exists() {
        println!("Output file already exists.");
        return;
    }
    create_dir_all(&output).unwrap();

    let spec = openapi_generator::read_spec(&input);
    openapi_generator::generate(spec, &output);

}
