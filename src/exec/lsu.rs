use crate::{data::{memory::Memory, rf_scalar::ScalarRF}, instruction_set::Instruction, exec::ScalarFU};

pub struct Lsu;

// LB, LH, LW, LBU, LHU, SB, SH, SW
impl ScalarFU for Lsu {
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory) {
        todo!()
    }
}
