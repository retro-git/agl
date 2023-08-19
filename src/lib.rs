pub mod compiler;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser); // synthesized by LALRPOP

use crate::compiler::compile_ffi;
use crate::compiler::Mode;
uniffi::include_scaffolding!("agl");