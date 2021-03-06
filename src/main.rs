#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc, "/calc/calc.rs");
mod calc_ast;
mod terminal;
mod readline;
mod sigaction;

use std::ptr;
use calc_ast::{Type, Term, Operator, MathError};
use libc::{SIGINT, siginfo_t, termios};
use sigaction::Sigaction;
use terminal::{Termios};
use readline::{readline};

// backed up terminal state, static because need to access from sigint handler
static mut BACKUP_TTY: *const termios = ptr::null();

fn main() {
    // Sets up interrupt handler for SIGINT
    let _interrupt_handler: Sigaction = Sigaction::new(SIGINT, sigint_handler).unwrap();

    // Backup tty to restore later
    let mut old_term: Termios = Termios::new().unwrap();
    old_term.backup_tty();
    unsafe {
        BACKUP_TTY = old_term.cstruct as *const termios;
    }

    // Set tty into raw mode
    Termios::set_raw();

    // Start parsing
    let parser = calc::ExprLineParser::new();
    loop {
        let input = readline();
        if let Ok(tree) = parser.parse(&input[..]) {
            let res = calculate(&tree.exp);
            match res {
                Ok(r) => {
                    match tree.format {
                        Type::Decimal => print!("{}\r\n", r),
                        Type::Hex => print!("{:#x}\r\n", r),
                        Type::Binary => print!("{:#b}\r\n", r),
                    }
                },
                Err(e) => {
                    match e {
                        MathError::BadShift => {
                            print!("You shifted more than 64 bits\r\n");
                        },
                    }
                }
            }
        } else {
            print!("syntax error\r\n");
        }
    }
}

fn calculate(t: &Term) -> Result<u64, MathError> {
    match t {
        Term::Line(t1, op, t2) => {
            let res = match op {
                Operator::Add => {
                    let l: u64 = calculate(t1)?;
                    let r: u64 = calculate(t2)?;
                    Ok(l.wrapping_add(r))
                },
                Operator::Subtract => {
                    let l: u64 = calculate(t1)?;
                    let r: u64 = calculate(t2)?;
                    Ok(l.wrapping_sub(r))
                },
                Operator::Mul => {
                    let l: u64 = calculate(t1)?;
                    let r: u64 = calculate(t2)?;
                    Ok(l.wrapping_mul(r))
                },
                Operator::Div => {
                    let l: u64 = calculate(t1)?;
                    let r: u64 = calculate(t2)?;
                    Ok(l.wrapping_div(r))
                },
                Operator::LShift => {
                    let l: u64 = calculate(t1)?;
                    let r: u64 = calculate(t2)?;
                    if r < 64 {
                        Ok(l << r)
                    } else {
                        Err(MathError::BadShift)
                    }
                },
                Operator::RShift => {
                    let l: u64 = calculate(t1)?;
                    let r: u64 = calculate(t2)?;
                    if r < 64 {
                        Ok(l >> r)
                    } else {
                        Err(MathError::BadShift)
                    }
                },
                Operator::Xor => {
                    let l: u64 = calculate(t1)?;
                    let r: u64 = calculate(t2)?;
                    Ok(l ^ r)
                },
                Operator::And => {
                    let l: u64 = calculate(t1)?;
                    let r: u64 = calculate(t2)?;
                    Ok(l & r)
                },
                Operator::Or => {
                    let l: u64 = calculate(t1)?;
                    let r: u64 = calculate(t2)?;
                    Ok(l | r)
                },
            };
            res
        },
        Term::Negate(t) => {
            let l: u64 = calculate(t)?;
            Ok(!l)
        }
        Term::Val(n) => Ok(*n),
    }
}

fn sigint_handler(_i: i32, _info: siginfo_t, _vp: usize) {
    unsafe { // accessing mutable static variable
        Termios::restore_tty(BACKUP_TTY);
    }
    std::process::exit(0);
}
