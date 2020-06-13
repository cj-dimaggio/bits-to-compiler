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

fn write_string_literal<W: Write> (literal: String, write_buffer: &mut BufWriter<W>) {
    for byte in literal.bytes() {
        if let Err(e) = write_buffer.write(&[byte]) {
            println!("Unable to write to output: {}", e);
            return;
        }
    }
}

pub fn create_binary(tokens: Vec<Token>, output_file: fs::File) {
    let mut write_buffer = BufWriter::new(output_file);

    for token in tokens {
        match token {
            Token::Binary(byte) => write_byte(byte, &mut write_buffer),
            Token::QuotedString(literal) => write_string_literal(literal, &mut write_buffer),
            _ => continue,
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

        assert_eq!(write_vec.len(), 1);
        assert_eq!(write_vec[0], 0b10110011);
    }

    #[test]
    fn writes_a_string_literal() {
        let mut write_vec = Vec::<u8>::new();
        {
            let mut buffer = BufWriter::new(Cursor::new(&mut write_vec));
        
            write_string_literal(String::from("FooBar"), &mut buffer);    
        }
        assert_eq!(write_vec.len(), 6);

        assert_eq!(write_vec[0], 'F' as u8);
        assert_eq!(write_vec[1], 'o' as u8);
        assert_eq!(write_vec[2], 'o' as u8);
        assert_eq!(write_vec[3], 'B' as u8);
        assert_eq!(write_vec[4], 'a' as u8);
        assert_eq!(write_vec[5], 'r' as u8);
    }
}
