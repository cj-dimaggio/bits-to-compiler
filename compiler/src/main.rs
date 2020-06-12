use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::io::Write;
use std::io::BufWriter;

fn compile(output_file: fs::File, input_contents: String) {
    let mut write_buffer = BufWriter::new(output_file);
    let mut current_byte: u8 = 0;
    let mut current_bit_position = 0;

    for character in input_contents.chars() {
        if character == '0' {
            // We actually don't need to do anything here because we initialize our
            // current_byte to 0, but if we wanted to be more explicit we could do
            // something like `current_byte &= ~bit`
        } else if character == '1' {
            let bit: u8 = 1 << (7 - current_bit_position);
            current_byte |= bit;
        } else {
            continue;
        }

        current_bit_position += 1;

        if current_bit_position > 7 {
           if let Err(e) = write_buffer.write(&[current_byte]) {
               println!("Unable to write to output: {}", e);
               return;
           }

           current_byte = 0;
           current_bit_position = 0;
        }
    }

    if let Err(e) = write_buffer.flush() {
        println!("Unable to write to output: {}", e);
    }

    eprintln!("Finished compiling");
}

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
        match fs::read_to_string(filename) {
            Ok(input_contents) => {
                let output_path = file_path.with_extension("bin");
                eprintln!("Opening output: {}", output_path.display());
                match fs::File::create(&output_path) {
                    Ok(output_file) => compile(output_file, input_contents),
                    Err(e) => println!("Could not open output file {}: {}", output_path.display(), e)
                }
            },
            Err(e) => println!("Could not open input file {}: {}", filename, e)
        }
    }
}
