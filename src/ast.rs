
use ::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Ast {
    pub expressions: Vec<Expr>,
}

#[derive(Debug, Serialize)]
pub enum Expr {
    Num(u64),
    Var(String),
    Dot { subject: Box<Expr>, attr: String },
    Function { subject: Box<Expr>, func: Box<Expr>, },
    Sum { left: Box<Expr>, right: Box<Expr> },
}
