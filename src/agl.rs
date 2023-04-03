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
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    N64,
    PSX
}

// pub fn instruction_to_gs(mode: Mode, instruction: &GSInstruction) -> String {
//     match mode {
//         Mode::N64 => {
//             match instruction {
//                 GSInstruction::Write8(addr, value) => { format!("80{:x}, 00{:x});", addr, value) },
//                 GSInstruction::Write16(addr, value) => { format!("gs_write16({}, {});", addr, value) },
//                 GSInstruction::Write32(addr, value) => { format!("gs_write32({}, {});", addr, value) },
//             }
//         }
//         Mode::PSX => {
//             match instruction {
//                 GSInstruction::Write8(addr, value) => { format!("gs_write8({}, {});", addr, value) },
//                 GSInstruction::Write16(addr, value) => { format!("gs_write16({}, {});", addr, value) },
//                 GSInstruction::Write32(addr, value) => { format!("gs_write32({}, {});", addr, value) },
//             }
//         }
//     }
// }