#[derive(Debug)]
pub struct Tree {
    pub format: Type,
    pub exp: Box<Term>,
}

#[derive(Debug)]
pub enum Term {
    Val(u64),
    Line(Box<Term>, Operator, Box<Term>),
    Negate(Box<Term>),
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Mul,
    Div,
    LShift,
    RShift,
    Xor,
}

#[derive(Debug)]
pub enum Type {
    Decimal,
    Hex,
    Binary,
}

pub enum MathError {
    BadShift,
}