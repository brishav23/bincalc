#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc, "/calc/calc.rs");

mod calc_ast;
use calc_ast::Type;

fn main() {
    loop {
        // let (n, t) = calc::ExprParser::new().parse("0x32 > b").expect("syntax error");
        if let Ok((n, t)) = calc::ExprLineParser::new().parse("0b1101 -> d") {
            match t {
                Type::Decimal => {
                    println!("{}", n);
                },
                Type::Binary => {
                    println!("{:#b}", n);
                },
                Type::Hex => {
                    println!("{:#x}", n);
                },
            }
        } else {
            println!("syntax error");
        }
    }
}