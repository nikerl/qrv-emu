use crate::{data::{memory::Memory, rf_scalar::ScalarRF}, instruction_set::Instruction, exec::ScalarFU};

pub struct Sys;

// ECALL, EBREAK, FENCE
impl ScalarFU for Sys {
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory) {
        todo!()
    }
}
