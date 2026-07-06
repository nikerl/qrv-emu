use crate::{data::{memory::Memory, rf_scalar::ScalarRF, rf_matrix::MatrixRF}, decoder::Instruction};

pub mod alu;
pub mod jump;
pub mod branch;
pub mod lsu;
pub mod matrix_lsu;
pub mod matrix_multiply;
pub mod sys;

pub trait ScalarFU {
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory);
}

pub trait MatrixFU {
    fn execute(instr: Instruction, scalar_regs: &mut ScalarRF, matrix_regs: &mut MatrixRF, mem: &mut Memory);
}
