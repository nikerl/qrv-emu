// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use crate::{
    data::rf_scalar::RegNames::*,
    instruction_set::{
        Instruction, 
        InstructionSet::*
    }, 
    exec::ExecutionUnit,
    system::SystemState
};

pub struct Jump;

// unconditional jump, return true
impl ExecutionUnit for Jump {
    fn execute(instr: Instruction, state: &mut SystemState) -> bool {
        let regs = &mut state.srf;

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
