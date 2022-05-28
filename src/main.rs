#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc, "/calc/calc.rs");
mod calc_ast;
mod terminal;
mod readline;
mod sigaction;

use std::ptr;
use calc_ast::{Type, Term, Operator};
use libc::{SIGINT, siginfo_t, termios};
use sigaction::Sigaction;
use terminal::{Termios};
use readline::{readline};

// backed up terminal state, static because need to access from sigint handler
static mut BACKUP: *const termios = ptr::null();

fn main() {
    // Sets up interrupt handler for SIGINT
    let _interrupt_handler: Sigaction = Sigaction::new(SIGINT, sigint_handler).unwrap();

    // Backup tty to restore later
    let mut old_term: Termios = Termios::new().unwrap();
    old_term.backup_tty();
    unsafe {
        BACKUP = old_term.cstruct as *const termios;
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
                    print!("Math error!\r\n");
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
                    let res: u128 = l as u128 + r as u128;
                    if res > 0xffffffffffffffff {
                        Err(MathError::BadAddition)
                    } else {
                        Ok(l + r)
                    }
                },
                Operator::Subtract => {
                    let l: u64 = calculate(t1)?;
                    let r: u64 = calculate(t2)?;
                    if l < r {
                        Err(MathError::BadSubtraction)
                    } else {
                        Ok(l - r)
                    }
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
                        Ok(l << r)
                    } else {
                        Err(MathError::BadShift)
                    }
                },
            };
            res
        },
        Term::Val(n) => Ok(*n),
    }
}

fn sigint_handler(_i: i32, _info: siginfo_t, _vp: usize) {
    unsafe { // accessing mutable static variable
        Termios::restore_tty(BACKUP);
    }
    std::process::exit(0);
}

enum MathError {
    BadSubtraction,
    BadAddition,
    BadShift,
}