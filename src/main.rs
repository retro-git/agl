use std::collections::HashMap;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

fn main() {
    println!("Hello, world!");
    parser::ProgramParser::new().parse(&HashMap::new(), "12").unwrap();
}

#[test]
fn test() {
    let parsed = parser::ProgramParser::new().parse(&HashMap::new(), "x = 14 + 7 * 2;").unwrap();
    // debug print
    println!("{:?}", parsed);
    assert!(false);
}