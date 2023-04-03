pub mod compiler;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser); // synthesized by LALRPOP