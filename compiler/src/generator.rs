use std::fs;
use std::io::{Write, BufWriter};
use super::tokenizer::Token;

fn write_byte<W: Write> (byte_array: [bool; 8], write_buffer: &mut BufWriter<W>) {
    let mut byte: u8 = 0;

    for i in 0..8 {
        if byte_array[i] {
            let bit: u8 = 1 << (7 - i);
            byte |= bit;
        }
    }

    if let Err(e) = write_buffer.write(&[byte]) {
        println!("Unable to write to output: {}", e);
        return;
    }
}

pub fn create_binary(tokens: Vec<Token>, output_file: fs::File) {
    let mut write_buffer = BufWriter::new(output_file);

    for token in tokens {
        match token {
            Token::BinaryByte(byte) => write_byte(byte, &mut write_buffer),
            Token::Comment(_) => continue
        }
    }

    if let Err(e) = write_buffer.flush() {
        println!("Unable to write to output: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor};

    #[test]
    fn writes_a_byte() {
        let mut write_vec = Vec::<u8>::new();
        {
            let mut buffer = BufWriter::new(Cursor::new(&mut write_vec));
        
            write_byte([true, false, true, true, false, false, true, true], &mut buffer);    
        }
        assert_eq!(write_vec[0], 0b10110011);
    }
}
