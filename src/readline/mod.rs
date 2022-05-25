use std::io;
use std::io::{Read, Write};

pub fn readline() -> String {
    // information about current line
    let mut line_buf = String::with_capacity(1024);
    let mut ins_buf = String::with_capacity(1024);
    // let mut cursor: usize = 0;
    // let mut line_length: usize = 0;

    let mut stdin: io::Stdin = io::stdin();
    let mut input_char = [0u8; 1];
    let char: &mut [u8] = &mut input_char[..];

    loop {
        stdin.read(char).unwrap();
        match char[0] {
            0x1bu8 => { // escape sequence
                let mut more_chars = [0u8; 2];
                let (first_char, second_char) = more_chars.split_at_mut(1);
                // let first_char = &mut more_chars[0..1];
                // let second_char = &mut more_chars[1..2];
                stdin.read(first_char).unwrap();
                stdin.read(second_char).unwrap();
    
                match (first_char[0], second_char[0]) {
                    (0x5bu8, 0x43u8) => { // right arrow
                        //
                    },
                    (0x5bu8, 0x44u8) => { // left arrow
                        //
                    },
                    (0x5bu8, 0x42u8) => { // down arrow
                        //
                    },
                    (0x5bu8, 0x41u8) => { // up arrow
                        //
                    },
                    _ => {}
                }
            },
            0xdu8 => {
                io::stdout().write(b"\r\n").unwrap();
                io::stdout().flush().unwrap();
                combine_bufs(&mut line_buf, &mut ins_buf);
                break;
            },
            _ => {
                line_buf.push(char[0] as char);
                io::stdout().write(char).unwrap();
                io::stdout().flush().unwrap();
            }
        }
    }

    line_buf
}

fn combine_bufs(line: &mut String, ins: &mut String) {
    loop {
        let char = ins.pop();
        match char {
            Some(c) => {line.push(c);},
            None => {break;},
        }
    }
}