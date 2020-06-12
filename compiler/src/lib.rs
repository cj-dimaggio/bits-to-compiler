use std::fs;

mod tokenizer;

pub fn compile(output_file: fs::File, input_contents: String) {
    tokenizer::tokenize(input_contents);
}
