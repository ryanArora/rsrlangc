use crate::ir::basic_block::BasicBlock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub basic_blocks: Vec<BasicBlock>,
}
