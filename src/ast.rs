// pub enum Expr {
//     Number(i32),
//     Op(Box<Expr>, BinOp, Box<Expr>),
// }

// pub enum BinOp {
//     Mul,
//     Div,
//     Add,
//     Sub,
// }

#[derive(Debug)]
pub enum Expr {
    Int(i32),
    Ident(String),
}

#[derive(Debug)]
pub enum Statement {
    Write8(Expr, Expr),
}

#[derive(Debug)]
pub enum Declaration {
    Assign(String, Expr),
    Statement(Statement),
}

pub struct Program {
    pub declarations: Vec<Declaration>,
}