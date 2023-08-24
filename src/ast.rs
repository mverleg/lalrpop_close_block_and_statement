
#[derive(Debug)]
pub struct Ast {
    pub expressions: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Num(u64),
    Var(String),
    Dot(Box<Expr>, String),
    Sum(Box<[Expr; 2]>),
}
