
#[derive(Debug)]
pub enum Expr {
    Nr(u64),
    Sum(Box<[Expr; 2]>),
}
