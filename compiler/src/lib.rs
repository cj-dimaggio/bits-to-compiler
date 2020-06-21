use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

mod tokenizer;
mod instructions;
mod generator;

pub fn compile(input_file: File, output_file: File) {
    let reader = BufReader::new(input_file);
    let mut instructions = Vec::<Box<dyn instructions::Instruction>>::new();
    let mut labels = HashMap::<String, u16>::new();
    let mut location: u16 = 0;

    for line in reader.lines() {
        let line = line.expect("Could not read from file");
        let mut tokens = tokenizer::tokenize(line).unwrap();

        // Handle label
        if let Some(tokenizer::Token::Label(label)) = tokens.get(0) {
            if let Some(_) = labels.insert(label.clone(), location) {
                println!("label {} defined more than once", label)
            }

            tokens.remove(0);
        }

        // If the line is nothing but a comment or a label
        if tokens.len() == 0 {
            continue;
        }

        let instruction = instructions::extract_instruction(&tokens).unwrap();
        location += instruction.byte_len();
        instructions.push(instruction);
    }
}
