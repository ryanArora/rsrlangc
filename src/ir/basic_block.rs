use crate::ir::{
    instruction::{Instruction, TerminatorInstruction},
    label::Label,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicBlock {
    pub label: Label,
    pub statements: Vec<Instruction>,
    pub terminator_instruction: TerminatorInstruction,
}
