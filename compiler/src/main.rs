use std::env;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    for filename in args[1..].iter() {
        let file_path = Path::new(filename);
        match file_path.extension().and_then(OsStr::to_str) {
            Some("bit") => (),
            Some(_) | None => {
                println!("Skipping '{}' as it's missing the expected '.bit' extension", filename);
                continue;
            },
        }

        eprintln!("Opening input: {}", filename);
        match File::open(filename) {
            Ok(input_file) => {
                let output_path = file_path.with_extension("bin");
                eprintln!("Opening output: {}", output_path.display());
                match File::create(&output_path) {
                    Ok(output_file) => compiler::compile(input_file, output_file),
                    Err(e) => println!("Could not open output file {}: {}", output_path.display(), e)
                }
            },
            Err(e) => println!("Could not open input file {}: {}", filename, e)
        }
    }
}
