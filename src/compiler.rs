use std::collections::HashMap;
use crate::parser::ProgramParser;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum BitWidth {
    Bit8,
    Bit16,
    Bit32,
}

#[derive(Debug, Clone, Copy)]
pub enum LogicalOp {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Copy)]
pub enum GSInstruction {
    Conditional(BitWidth, LogicalOp, i32, i32),
    Write(BitWidth, i32, i32),
    Incr(BitWidth, i32, i32),
    Decr(BitWidth, i32, i32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
#[wasm_bindgen]
pub enum Mode {
    PSX,
    N64
}

#[wasm_bindgen]
pub fn compile(source: String, mode: Mode) -> String {
    let parsed = ProgramParser::new().parse(&mut HashMap::new(), &source).unwrap();
    parsed.iter().map(|instruction| { instruction_to_string(mode, *instruction) }).collect::<Vec<String>>().join("\n")
}

fn instruction_to_string(mode: Mode, instruction: GSInstruction) -> String {
    use GSInstruction::*;
    use BitWidth::*;
    use LogicalOp::*;
    use Mode::*;
    match instruction {
        Write(width, addr, value) => {
            match (mode, width) {
                //example: addr = 0xffff and value = 0x12: 3000ffff 0012
                (PSX, Bit8) => format!("30{:06x} {:04x}", addr & 0xffffff, value & 0xff),
                (PSX, Bit16) => format!("80{:06x} {:04x}", addr & 0xffffff, value & 0xffff),
                (PSX, Bit32) => panic!("PSX does not support 32-bit writes"),

                (N64, Bit8) => format!("80{:06x} {:04x}", addr & 0xffffff, value & 0xff),
                (N64, Bit16) => format!("81{:06x} {:04x}", addr & 0xffffff, value & 0xffff),
                (N64, Bit32) => panic!("N64 does not support 32-bit writes"),
            }
        },
       Conditional(width, op, addr, value) => {
            match (mode, width, op) {
                (PSX, Bit8, Equal)          => format!("E0{:06x} {:04x}", addr & 0xffffff, value & 0xff),
                (PSX, Bit8, NotEqual)       => format!("E1{:06x} {:04x}", addr & 0xffffff, value & 0xff),
                (PSX, Bit8, LessThan)       => format!("E2{:06x} {:04x}", addr & 0xffffff, value & 0xff),
                (PSX, Bit8, GreaterThan)    => format!("E3{:06x} {:04x}", addr & 0xffffff, value & 0xff),
                (PSX, Bit16, Equal)         => format!("D0{:06x} {:04x}", addr & 0xffffff, value & 0xffff),
                (PSX, Bit16, NotEqual)      => format!("D1{:06x} {:04x}", addr & 0xffffff, value & 0xffff),
                (PSX, Bit16, LessThan)      => format!("D2{:06x} {:04x}", addr & 0xffffff, value & 0xffff),
                (PSX, Bit16, GreaterThan)   => format!("D3{:06x} {:04x}", addr & 0xffffff, value & 0xffff),
                (PSX, Bit32, _)             => panic!("PSX does not support 32-bit conditionals"),

                (N64, Bit8, Equal)          => format!("D0{:06x} {:04x}", addr & 0xffffff, value & 0xff),
                (N64, Bit8, NotEqual)       => format!("D2{:06x} {:04x}", addr & 0xffffff, value & 0xff),
                // (N64, Bit8, LessThan)       => format!("gs_conditional8_lt({}, {})", addr, value),
                // (N64, Bit8, GreaterThan)    => format!("gs_conditional8_gt({}, {})", addr, value),
                (N64, Bit16, Equal)         => format!("D1{:06x} {:04x}", addr & 0xffffff, value & 0xff),
                (N64, Bit16, NotEqual)      => format!("D3{:06x} {:04x}", addr & 0xffffff, value & 0xff),
                // (N64, Bit16, LessThan)      => format!("gs_conditional16_lt({}, {})", addr, value),
                // (N64, Bit16, GreaterThan)   => format!("gs_conditional16_gt({}, {})", addr, value),
                (N64, Bit32, _)             => panic!("N64 does not support 32-bit conditionals"),
                (N64, _, _)                 => panic!("N64 does not support greater or lesser than conditionals"),
            }
        }
        _ => todo!()
    }
}
