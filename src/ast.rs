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

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Ident(String),
    Op(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Debug)]
pub enum Statement {
    Write8(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Declaration {
    Assign(String, Box<Expr>),
    Statement(Statement),
}

// #[derive(Debug)]
// pub struct Program {
//     pub declarations: Vec<Declaration>,
// }