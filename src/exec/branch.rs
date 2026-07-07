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


pub struct Branch;

// BEQ, BNE, BLT, BGE, BLTU, BGEU
impl ScalarFU for Branch {
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory) {
        let offset = instr.im1;
        let rs1 = instr.rs1 as usize;
        let rs2 = instr.rs2 as usize;

        match instr.opcode {
            BEQ => {
                if regs[rs1] == regs[rs2] {
                    regs.pc = (regs.pc as i32).wrapping_add(offset) as u32;
                }
            }
            BNE => {
                if regs[rs1] != regs[rs2] {
                    regs.pc = (regs.pc as i32).wrapping_add(offset) as u32;
                }
            }
            BLT => {
                if (regs[rs1] as i32) < (regs[rs2] as i32) {
                    regs.pc = (regs.pc as i32).wrapping_add(offset) as u32;
                }
            }
            BGE => {
                if (regs[rs1] as i32) >= (regs[rs2] as i32) {
                    regs.pc = (regs.pc as i32).wrapping_add(offset) as u32;
                }
            }
            BLTU => {
                if regs[rs1] < regs[rs2] {
                    regs.pc = regs.pc.wrapping_add(offset as u32);
                }
            }
            BGEU => {
                if regs[rs1] >= regs[rs2] {
                    regs.pc = regs.pc.wrapping_add(offset as u32);
                }
            }
            _ => println!("Unrecognized opcode")
        }
    }
}
