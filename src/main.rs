use std::collections::HashMap;
use agl_rs::ast::*;
use std::fs;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

fn main() {
    println!("Hello, world!");
    
}

#[test]
fn test() {
    let mut output: Vec<GSInstruction> = Vec::new();
    //load code from /agl/block.agl
    let code = fs::read_to_string("agl/block.agl").unwrap();
    parser::ProgramParser::new().parse(&mut HashMap::new(), &mut output, &code).unwrap();
    println!("{:?}", output);
    assert!(false);
}