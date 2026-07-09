use crate::{
    data::{
        memory::Memory, 
        rf_scalar::{ScalarRF, RegNames::*}
    }, 
    instruction_set::{
        Instruction, 
        InstructionSet::*
    }, 
    exec::ScalarFU
};

pub struct Branch;

// Returns bool: branch taken (true), or not taken (false)
impl ScalarFU for Branch {
    fn execute(instr: Instruction, regs: &mut ScalarRF, _mem: &mut Memory) -> bool {
        let offset = instr.im1;
        let rs1 = instr.rs1 as usize;
        let rs2 = instr.rs2 as usize;

        match instr.opcode {
            BEQ => {
                if regs[rs1] == regs[rs2] {
                    regs[PC] = (regs[PC] as i32).wrapping_add(offset) as u32;
                    return true;
                }
            }
            BNE => {
                if regs[rs1] != regs[rs2] {
                    regs[PC] = (regs[PC] as i32).wrapping_add(offset) as u32;
                    return true;
                }
            }
            BLT => {
                if (regs[rs1] as i32) < (regs[rs2] as i32) {
                    regs[PC] = (regs[PC] as i32).wrapping_add(offset) as u32;
                    return true;
                }
            }
            BGE => {
                if (regs[rs1] as i32) >= (regs[rs2] as i32) {
                    regs[PC] = (regs[PC] as i32).wrapping_add(offset) as u32;
                    return true;
                }
            }
            BLTU => {
                if regs[rs1] < regs[rs2] {
                    regs[PC] = regs[PC].wrapping_add(offset as u32);
                    return true;
                }
            }
            BGEU => {
                if regs[rs1] >= regs[rs2] {
                    regs[PC] = regs[PC].wrapping_add(offset as u32);
                    return true;
                }
            }
            _ => println!("Unrecognized opcode")
        }
        
        return false;
    }
}
