use std::collections::HashMap;
use agl_rs::agl::*;
use std::fs;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

fn main() {
    println!("Hello, world!");
    
}

#[test]
fn test() {
    //load code from /agl/block.agl
    let code = fs::read_to_string("agl/block.agl").unwrap();
    let parsed = parser::ProgramParser::new().parse(&mut HashMap::new(), &code).unwrap();
    println!("{:?}", parsed);
    assert!(false);
}