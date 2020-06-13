use std::fs;

mod tokenizer;
mod ast;
mod generator;

pub fn compile(output_file: fs::File, input_contents: String) {
    match tokenizer::tokenize(input_contents) {
        Err(e) => {
            println!("Compilation error: {:?}", e);
            return;
        },
        Ok(tokens) => generator::create_binary(tokens, output_file),
    }
}
