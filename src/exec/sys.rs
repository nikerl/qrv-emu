use crate::{data::{memory::Memory, rf_scalar::ScalarRF}, decoder::Instruction, exec::ScalarFU};

pub struct Sys;

// ECALL, EBREAK, FENCE
impl ScalarFU for Sys {
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory) {
        todo!()
    }
}
