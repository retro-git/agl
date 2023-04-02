

#[derive(Debug, Clone, Copy)]
pub enum GSInstruction {
    Write8(i32, i32),
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    N64,
    PSX
}