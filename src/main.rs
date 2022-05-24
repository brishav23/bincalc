use std::io;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc, "/calc/calc.rs");

mod calc_ast;
use calc_ast::{Type, Term, Operator};

fn main() {
    let parser = calc::ExprLineParser::new();
    let mut input: String = String::with_capacity(1024);
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Can't read stdin");
        if let Ok(tree) = parser.parse(&input[..]) {
            let res: u64 = calculate(&tree.exp);
            match tree.format {
                Type::Decimal => println!("{}", res),
                Type::Hex => println!("{:#x}", res),
                Type::Binary => println!("{:#b}", res),
            }
        } else {
            println!("syntax error");
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
