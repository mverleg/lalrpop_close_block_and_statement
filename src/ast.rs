
use ::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Ast {
    pub expressions: Vec<Expr>,
}

#[derive(Debug, Serialize)]
pub enum Expr {
    Num(u64),
    Var(String),
    Dot(Box<Expr>, String),
    Function(Box<[Expr; 2]>),
    Sum(Box<[Expr; 2]>),
}
