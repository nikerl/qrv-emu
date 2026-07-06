use crate::{data::{memory::Memory, rf_scalar::ScalarRF}, instruction_set::Instruction, exec::ScalarFU};

pub struct Jump;

// JALR, JAL
impl ScalarFU for Jump {
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory) {
        todo!()
    }
}
