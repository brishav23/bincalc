#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc, "/calc/calc.rs");

mod calc_ast;
use calc_ast::{Term, Operator};

fn main() {
    let parser = calc::ExprLineParser::new();
    loop {
        if let Ok(tree) = parser.parse("((0b111 + 0x23) + 23) -> d") {
            // println!("{:?}", tree);
            let res: u64 = calculate(&*tree.exp);
            println!("{}", res);
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
                    calculate(&*t1) + calculate(&*t2)
                },
                Operator::Subtract => {
                    calculate(&*t1) - calculate(&*t2)
                },
            };
            res
        },
        Term::Val(n) => *n,
    }
}