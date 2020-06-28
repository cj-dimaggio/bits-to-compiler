use std::fs::File;
use std::io::BufWriter;
use std::io::Read;

mod tokenizer;

pub fn compile(mut input_file: File, output_file: File) {
    let mut code = String::new();
    if let Err(e) = input_file.read_to_string(&mut code) {
        println!("Unable to read file: {}", e);
    }

    let mut writer = BufWriter::new(output_file);

    tokenizer::tokenize(code);
}
