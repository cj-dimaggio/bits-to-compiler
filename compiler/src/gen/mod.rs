use std::fs::File;
use std::io::{ BufWriter, Write };
use super::parser::Program;
use super::parser::function::Function;
use super::parser::statement::Statement;
use super::parser::expression::Expression;
use std::collections::HashMap;

struct Context<'a> {
    writer: &'a mut BufWriter<File>,
    strings: HashMap<String, String>,
    variables: HashMap<String, usize>, // stack offset
    label_counter: usize,
}

impl Context<'_> {
    pub fn new(writer: &mut BufWriter<File>) -> Context {
        Context {
            writer,
            strings: HashMap::new(),
            variables: HashMap::new(),
            label_counter: 0,
        }
    }

    pub fn write(&mut self, data: &str) {
        self.writer.write(data.as_bytes()).expect("Unable to write");
        self.writer.write(&['\n' as u8]).expect("Unable to write");
    }

    pub fn get_string(&mut self, data: &str) -> String {
        let data = data.to_string();

        if let Some(label) = self.strings.get(&data) {
            label.clone()
        } else {
            let label = format!("string_{}", self.strings.len());
            self.strings.insert(data, label.clone());
            label
        }
    }

    pub fn get_variable_offset(&mut self, name: &str) -> Option<&usize> {
        self.variables.get(name)
    }

    pub fn new_variable(&mut self, name: &str) {
        let length = self.variables.len();
        self.variables.insert(name.to_string(), length);
    }

    pub fn new_label(&mut self) -> String {
        let label = format!(".label_{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    pub fn new_function(&mut self) {
        self.variables.clear();
        self.label_counter = 0;
    }

    pub fn write_strings(&mut self) {
        let mut strings = vec![];
        for (data, label) in &self.strings {
            strings.push(format!("{}: db \"{}\", 0", label, data));
        }

        // Multiple loops just to placate the borrow checker
        for s in strings {
            self.write(&s);
        }
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

    // Built in functions
    ctx.write(r#"
print:
    push bp
    mov bp, sp

    mov ah,0x0e
    int 0x10

    mov sp, bp
    pop bp
    ret
    "#)
}

fn epilogue(ctx: &mut Context) {
    ctx.write("epilogue:");
    ctx.write("cli");
    ctx.write("hlt");

    ctx.write_strings();

    ctx.write("times 510 - ($-$$) db 0");
    ctx.write("dw 0xaa55");
}

pub fn generate(writer: &mut BufWriter<File>, program: Program) {
    let mut ctx = Context::new(writer);

    prologue(&mut ctx);

    for function in program.functions {
        compile_function(&mut ctx, &function);
    }
    
    epilogue(&mut ctx);
}

fn compile_function(ctx: &mut Context, function: &Function) {
    ctx.new_function();
    ctx.write(&format!("{}:", function.identifier));

    // Normalize stack
    ctx.write("push bp");
    ctx.write("mov bp, sp");

    if let Some(arg) = &function.argument {
        ctx.write("push ax");
        ctx.new_variable(arg);  
    }

    for statement in &function.statements {
        compile_statement(ctx, &statement);
    }

    ctx.write("mov sp, bp");
    ctx.write("pop bp");
    ctx.write("ret");
}

fn compile_expression(ctx: &mut Context, expression: &Expression) {
    match expression {
        Expression::NumberLiteral(num) => {
            ctx.write(&format!("mov ax, {}", num));
        },
        Expression::StringLiteral(data) => {
            let string = ctx.get_string(data);
            ctx.write(&format!("mov ax, {}", string));
        },
        Expression::Variable(name) => {
            let offset = *ctx.get_variable_offset(name).expect("Undefined variable");
            ctx.write(&format!("mov ax, [bp - {}]", (2 * (offset + 1)) ))
        }
        Expression::Addition { left, right } => {
            compile_expression(ctx, left);
            ctx.write("mov bx, ax");
            compile_expression(ctx, right);

            ctx.write("add ax, bx");
        },
        Expression::Lookup { base, index } => {
            compile_expression(ctx, base);
            ctx.write("mov bx, ax");
            compile_expression(ctx, index);

            // Lookups are only available for single bits
            // Scale not available with 16 bit registers
            ctx.write("mov al, [ebx + eax]");
        },
        Expression::NotComparison { left, right } => {
            compile_expression(ctx, left);
            ctx.write("mov bx, ax");
            compile_expression(ctx, right);

            ctx.write("cmp ax, bx");
            ctx.write("mov ax, 0");
            ctx.write("setnz al");
        }
    }
}

fn compile_statement(ctx: &mut Context, statement: &Statement) {
    match statement {
        Statement::Assignment { identifier, value } => {
            compile_expression(ctx, value);

            match ctx.get_variable_offset(identifier) {
                Some(offset) => {
                    let offset = *offset;
                    ctx.write(&format!("mov [bp - {}], ax", (2 * (offset + 1)) ));
                },
                None => {
                    ctx.write("push ax");
                    ctx.new_variable(identifier);        
                }
            }
        },
        Statement::FunctionCall { identifier, param } => {
            // Only handle a max of one param for now
            if let Some(e) = param {
                compile_expression(ctx, e);
            }

            ctx.write(&format!("call {}", identifier));
        },
        Statement::While { condition, statements } => {
            let loop_label = ctx.new_label();
            let end_label = ctx.new_label();
            ctx.write(&format!("{}:", loop_label));

            compile_expression(ctx, condition);
            ctx.write("cmp ax, 0");
            ctx.write(&format!("je {}", end_label));

            for s in statements {
                compile_statement(ctx, s);
            }

            ctx.write(&format!("jmp {}", loop_label));
            ctx.write(&format!("{}:", end_label));
        }
    }
}
