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
    system::SystemState,
    trap::TrapCause
};

pub struct Branch;

// Returns bool: branch taken (true), or not taken (false)
impl ExecutionUnit for Branch {
    fn execute(instr: Instruction, state: &mut SystemState) -> Result<bool, TrapCause> {
        let regs = &mut state.srf;

        let offset = instr.im1;
        let rs1 = instr.rs1 as usize;
        let rs2 = instr.rs2 as usize;

        match instr.opcode {
            BEQ => {
                if regs[rs1] == regs[rs2] {
                    regs[PC] = (regs[PC] as i32).wrapping_add(offset) as u32;
                    return Ok(true);
                }
            }
            BNE => {
                if regs[rs1] != regs[rs2] {
                    regs[PC] = (regs[PC] as i32).wrapping_add(offset) as u32;
                    return Ok(true);
                }
            }
            BLT => {
                if (regs[rs1] as i32) < (regs[rs2] as i32) {
                    regs[PC] = (regs[PC] as i32).wrapping_add(offset) as u32;
                    return Ok(true);
                }
            }
            BGE => {
                if (regs[rs1] as i32) >= (regs[rs2] as i32) {
                    regs[PC] = (regs[PC] as i32).wrapping_add(offset) as u32;
                    return Ok(true);
                }
            }
            BLTU => {
                if regs[rs1] < regs[rs2] {
                    regs[PC] = regs[PC].wrapping_add(offset as u32);
                    return Ok(true);
                }
            }
            BGEU => {
                if regs[rs1] >= regs[rs2] {
                    regs[PC] = regs[PC].wrapping_add(offset as u32);
                    return Ok(true);
                }
            }
            _ => unreachable!("Decoder guarantees valid instructions")
        }
        
        return Ok(false);
    }
}
