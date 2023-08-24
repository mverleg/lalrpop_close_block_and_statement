
#[derive(Debug)]
pub struct Ast {
    pub expressions: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Num(u64),
    Sum(Box<[Expr; 2]>),
}
