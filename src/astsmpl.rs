#[derive(Debug)]
pub enum Statement {
    Write8(i32, i32),
}

#[derive(Debug)]
pub enum Declaration {
    Assign(String, i32),
    Statement(Statement),
}

// #[derive(Debug)]
// pub struct Program {
//     pub declarations: Vec<Declaration>,
// }