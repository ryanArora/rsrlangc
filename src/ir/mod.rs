mod basic_block;
mod function;
mod instruction;
mod label;
mod register;

use crate::ir::function::Function;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntermediateRepresentation {
    pub main_function: Function,
}
