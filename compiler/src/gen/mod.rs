use std::fs::File;
use std::io::{ BufWriter, Write };
use super::parser::Program;

mod intermediate;
mod instructions;

struct Context<'a> {
    writer: &'a mut BufWriter<File>,
    pub position: usize
}

impl Context<'_> {
    fn write(&mut self, bytes: &Vec::<u8>) {
        self.writer.write(&bytes).unwrap();
        self.position += bytes.len();
    }
}

pub fn generate(writer: &mut BufWriter<File>, program: Program) {
    let mut context = Context {
        writer,
        position: 0
    };

    context.write(&instructions::hlt());

    for _ in context.position..510 {
        context.write(&vec![0])
    }

    context.write(&vec![0x55, 0xaa])
}
