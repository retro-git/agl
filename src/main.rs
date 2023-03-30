#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

fn main() {
    println!("Hello, world!");
    parser::ProgramParser::new().parse("12").unwrap();
}

#[test]
fn test() {
    let parsed = parser::ProgramParser::new().parse("x = 13; write8 xyz 24;").unwrap();
    // debug print
    println!("{:?}", parsed);
    assert!(false);
}