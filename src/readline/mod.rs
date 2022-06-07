use std::io;
use std::io::{Read, Write};

pub fn readline() -> String {
    // information about current line
    let mut line_buf = String::with_capacity(1024);
    let mut ins_buf = String::with_capacity(1024);

    let mut stdin: io::Stdin = io::stdin();
    let mut input_char = [0u8; 1];
    let char: &mut [u8] = &mut input_char[..];

    loop {
        stdin.read(char).unwrap();
        match char[0] {
            0x1bu8 => { // escape sequence, read 1 or 2 more characters
                let mut more_chars = [0u8; 2];
                let (first_char, second_char) = more_chars.split_at_mut(1);

                // check if first char means something
                stdin.read(first_char).unwrap();
                match first_char[0] {
                    0x62 => {
                        if let Some(c) = line_buf.pop() {
                            ins_buf.push(c);
                            loop {
                                if let Some(c) = line_buf.pop() {
                                    if c == ' ' {
                                        line_buf.push(c);
                                        break;
                                    } else {
                                        ins_buf.push(c);
                                    }
                                } else {
                                    break;
                                }
                            }
                            put_line(&line_buf, &ins_buf);
                        }
                        continue; // don't read second char after escape
                    },
                    0x66 => {
                        loop {
                            if let Some(c) = ins_buf.pop() {
                                if c == ' ' {
                                    line_buf.push(c);
                                    break;
                                } else {
                                    line_buf.push(c);
                                }
                            } else {
                                break;
                            }
                        }
                        put_line(&line_buf, &ins_buf);
                        continue; // don't read second char after escape
                    },
                    _ => {},
                }

                // read second char if first char wasn't meaningful
                stdin.read(second_char).unwrap();
                match (first_char[0], second_char[0]) {
                    (0x5bu8, 0x43u8) => { // right arrow
                        let c = ins_buf.pop();
                        match c {
                            Some(c) => {
                                line_buf.push(c);
                            },
                            None => {},
                        }
                        put_line(&line_buf, &ins_buf);
                    },
                    (0x5bu8, 0x44u8) => { // left arrow
                        let c = line_buf.pop();
                        match c {
                            Some(c) => {
                                ins_buf.push(c);
                            },
                            None => {},
                        }
                        put_line(&line_buf, &ins_buf);
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
            0xdu8 => { // end of line
                io::stdout().write(b"\r\n").unwrap();
                io::stdout().flush().unwrap();
                combine_bufs(&mut line_buf, &mut ins_buf);
                break;
            },
            0x8u8 | 0x7fu8 => {
                let c = line_buf.pop();
                match c {
                    Some(_) => {
                        put_line(&line_buf, &ins_buf);
                    },
                    None => {},
                }
            },
            _ => {
                line_buf.push(char[0] as char);
                put_line(&line_buf, &ins_buf);
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

fn put_line(line: &String, ins: &String) {
    io::stdout().write(b"\x1b[2K\r").unwrap();
    io::stdout().write(line[..].as_bytes()).unwrap();
    let l = ins.len();
    if l > 0 {
        ins[..].as_bytes()
               .iter()
               .rev()
               .map(|x| x)
               .for_each(|x| {
                   io::stdout().write(&[*x]).unwrap();
                });
        for _ in 1..l+1 {
            io::stdout().write(b"\x1b[1D").unwrap();
        }
    }
    io::stdout().flush().unwrap();
}
