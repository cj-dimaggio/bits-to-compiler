use std::fs::File;
use std::io::BufWriter;
use std::io::Read;

mod tokenizer;
mod parser;

pub fn compile(mut input_file: File, output_file: File) {
    let mut code = String::new();
    if let Err(e) = input_file.read_to_string(&mut code) {
        println!("Unable to read file: {}", e);
    }

    let mut writer = BufWriter::new(output_file);

    match tokenizer::tokenize(code) {
        Ok(tokens) => {
            let program = parser::parse(tokens);
        },
        Err(e) => println!("Error tokenizing: {:?}", e)
    }
}
