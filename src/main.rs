use std::collections::HashMap;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP
lalrpop_mod!(pub parsersmpl);

fn main() {
    println!("Hello, world!");
    parser::ProgramParser::new().parse("12").unwrap();
}

#[test]
fn test() {
    let parsed = parser::ProgramParser::new().parse("x = 14 + 7 * 2; write8 x / 2 + 2 4;").unwrap();
    println!("{:?}", parsed);
    let parsed = parsersmpl::ProgramParser::new().parse(&mut HashMap::new(), "x = 14 + 7 * 2; write8 x / 2 + 2 4;").unwrap();
    println!("{:?}", parsed);
    assert!(false);
}