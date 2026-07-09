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

pub struct Jump;

// unconditional jump, return true
impl ScalarFU for Jump {
    fn execute(instr: Instruction, regs: &mut ScalarRF, _mem: &mut Memory) -> bool {
        let offset = instr.im1;
        let rs1 = instr.rs1 as usize;
        let rd = instr.rd as usize;

        match instr.opcode {
            JAL => {
                regs[rd] = regs[PC].wrapping_add(4);
                regs[PC] = (regs[PC] as i32).wrapping_add(offset) as u32;
            }
            JALR => {
                regs[rd] = regs[PC].wrapping_add(4);
                regs[PC] = ((regs[rs1] as i32).wrapping_add(offset) as u32) & !1;
            }
            _ => println!("Unrecognized opcode")
        }

        return true;
    }
}
