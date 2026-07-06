use crate::{data::{memory::Memory, rf_scalar::ScalarRF}, decoder::Instruction, exec::ScalarFU};

pub struct Branch;

// BEQ, BNE, BLT, BGE, BLTU, BGEU
impl ScalarFU for Branch {
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory) {
        todo!()
    }
}
