use crate::ir::{
    label::Label,
    register::{FloatRegister, IntegerRegister},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    IntegerAdd {
        rd: IntegerRegister,
        rs1: IntegerRegister,
        rs2: IntegerRegister,
    },
    FloatAdd {
        rd: FloatRegister,
        rs1: FloatRegister,
        rs2: FloatRegister,
    },
    Branch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminatorInstruction {
    Return,
    Branch { label: Label },
}
