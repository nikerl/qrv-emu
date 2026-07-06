use crate::{data::{memory::Memory, rf_scalar::ScalarRF, rf_matrix::MatrixRF}, decoder::Instruction, exec::MatrixFU};

pub struct MatrixLSU;

// MZERO, MLDW, MSTW, SPLDW, DLDW
impl MatrixFU for MatrixLSU {
    fn execute(instr: Instruction, scalar_regs: &mut ScalarRF, matrix_regs: &mut MatrixRF, mem: &mut Memory) {
        todo!()
    }
}
