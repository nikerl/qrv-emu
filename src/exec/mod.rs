use crate::{data::{memory::Memory, rf_scalar::ScalarRF, rf_matrix::MatrixRF}, instruction_set::Instruction};

pub mod alu;
pub mod jump;
pub mod branch;
pub mod lsu;
pub mod matrix_lsu;
pub mod matrix_multiply;
pub mod sys;

/// Execute returns bool: branch taken (true), or not taken (false). To skip PC increment after branch
/// 
/// Only applicable for branch and jump type instructions, all others should return false
pub trait ScalarFU {
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory) -> bool;
}

/// Execute returns bool: branch taken (true), or not taken (false). To skip PC increment after branch
/// 
/// Only applicable for branch and jump type instructions, all others should return false
pub trait MatrixFU {
    fn execute(instr: Instruction, scalar_regs: &mut ScalarRF, matrix_regs: &mut MatrixRF, mem: &mut Memory) -> bool;
}
