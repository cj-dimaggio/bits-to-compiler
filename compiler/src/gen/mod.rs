use std::fs::File;
use std::io::{ BufWriter, Write };
use super::parser::Program;
use super::parser::function::Function;
use super::parser::statement::Statement;
use super::parser::expression::Expression;

struct Context<'a> {
    writer: &'a mut BufWriter<File>
}

impl Context<'_> {
    pub fn write(&mut self, data: &str) {
        self.writer.write(data.as_bytes()).expect("Unable to write");
        self.writer.write(&['\n' as u8]).expect("Unable to write");
    }
}

fn prologue(ctx: &mut Context) {
    ctx.write("bits 16");
    ctx.write("org 0x7c00");

    ctx.write("prologue:");

    // Setup the stack
    ctx.write("mov bp, ($$ + 510)");
    ctx.write("mov sp, ($$ + 510)");
    ctx.write("call main");
    ctx.write("call epilogue");
}

fn epilogue(ctx: &mut Context) {
    ctx.write("epilogue:");
    ctx.write("cli");
    ctx.write("hlt");

    ctx.write("times 510 - ($-$$) db 0");
    ctx.write("dw 0xaa55");
}

pub fn generate(writer: &mut BufWriter<File>, program: Program) {
    let mut ctx = Context{
        writer,
    };

    prologue(&mut ctx);

    for function in program.functions {
        compile_function(&mut ctx, &function);
    }
    
    epilogue(&mut ctx);
}

fn compile_function(ctx: &mut Context, function: &Function) {
    ctx.write(&format!("{}:", function.identifier));
    for statement in &function.statements {
    }
    ctx.write("ret");
}
