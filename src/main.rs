#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc, "/calc/calc.rs");
mod calc_ast;
mod terminal;
mod readline;

use calc_ast::{Type, Term, Operator};
use terminal::{Termios};
use readline::{readline};

fn main() {
    // Backup tty to restore later
    let mut old_term: Termios = Termios::new().unwrap();
    old_term.backup_tty();

    // Set tty into raw mode
    Termios::set_raw();

    let parser = calc::ExprLineParser::new();
    loop {
        let input = readline();
        if let Ok(tree) = parser.parse(&input[..]) {
            let res: u64 = calculate(&tree.exp);
            match tree.format {
                Type::Decimal => print!("{}\r\n", res),
                Type::Hex => print!("{:#x}\r\n", res),
                Type::Binary => print!("{:#b}\r\n", res),
            }
        } else {
            print!("syntax error\r\n");
        }
    }
}

fn calculate(t: &Term) -> u64 {
    match t {
        Term::Line(t1, op, t2) => {
            let res = match op {
                Operator::Add => {
                    calculate(t1) + calculate(t2)
                },
                Operator::Subtract => {
                    calculate(t1) - calculate(t2)
                },
            };
            res
        },
        Term::Val(n) => *n,
    }
}
