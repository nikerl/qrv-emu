use crate::{
    data::{
        memory::Memory, 
        rf_scalar::ScalarRF
    }, 
    instruction_set::{
        Instruction, 
        InstructionSet::*
    }, 
    exec::ScalarFU
};

pub struct Jump;

// JALR, JAL
impl ScalarFU for Jump {
    fn execute(instr: Instruction, regs: &mut ScalarRF, _mem: &mut Memory) {
        let offset = instr.im1;
        let rs1 = instr.rs1 as usize;
        let rd = instr.rd as usize;

        match instr.opcode {
            JAL => {
                regs[rd] = regs.pc.wrapping_add(4);
                regs.pc = (regs.pc as i32).wrapping_add(offset) as u32;
            }
            JALR => {
                regs[rd] = regs.pc.wrapping_add(4);
                regs.pc = ((regs[rs1] as i32).wrapping_add(offset) as u32) & !1;
            }
            _ => println!("Unrecognized opcode")
        }
    }
}
