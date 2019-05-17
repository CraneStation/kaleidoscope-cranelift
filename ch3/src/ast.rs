#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum BinaryOp {
    LessThan,
    Minus,
    Plus,
    Times,
}

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    Number(f64),
    Variable(String),
}

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Expr,
}

#[derive(Debug)]
pub struct Prototype {
    pub function_name: String,
    pub parameters: Vec<String>,
}
