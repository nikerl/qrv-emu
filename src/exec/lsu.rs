// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use crate::{
    instruction_set::{
        Instruction, 
        InstructionSet::*
    }, 
    exec::{
        ExecutionUnit,
        ExecResult
    },
    system::SystemState,
    trap::TrapCause
};

pub struct Lsu;

impl ExecutionUnit for Lsu {
    fn execute(instr: Instruction, state: &mut SystemState) -> Result<ExecResult, TrapCause> {
        let mem = &mut state.mem;
        let regs = &mut state.srf;
        

        let rd = instr.rd as usize;
        let addr = (regs[instr.rs1 as usize].wrapping_add(instr.im1 as u32)) as usize;
        let val = regs[instr.rs2 as usize];
        
        match instr.opcode {
            LB => regs[rd] = mem.load_byte(addr)? as i8 as i32 as u32,
            LH => regs[rd] = mem.load_half(addr)? as i16 as i32 as u32,
            LW => regs[rd] = mem.load_word(addr)? as i32 as u32,
            LBU => regs[rd] = mem.load_byte(addr)? as u32,
            LHU => regs[rd] = mem.load_half(addr)? as u32,
            SB => mem.store_byte(addr, val as u8)?,
            SH => mem.store_half(addr, val as u16)?,
            SW => match mem.store_word(addr, val as u32)? {
                ExecResult::Continue { .. } => {},
                ExecResult::Exit { exit_status } => return Ok(ExecResult::Exit { exit_status })
            }
            _ => unreachable!("Decoder guarantees valid instructions")
        }

        return Ok(ExecResult::Continue { branch_taken: false });
    }
}
