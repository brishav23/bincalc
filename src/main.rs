#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calc);

fn main() {
    println!("{}", calc::TermParser::new().parse("0x32").unwrap());
}

#[test]
fn correct_casting() {
    assert_eq!(calc::TermParser::new().parse("-23").unwrap(), 18446744073709551593u64);
    assert_eq!(calc::TermParser::new().parse("23").unwrap(), 23u64);
    assert_eq!(calc::TermParser::new().parse("0x52").unwrap(), 82u64);
}