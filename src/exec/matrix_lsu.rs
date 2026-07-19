// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use crate::{
    instruction_set::{
        Instruction, 
        InstructionSet::*
    }, 
    exec::ExecutionUnit,
    system::SystemState,
    trap::TrapCause
};


pub struct MatrixLSU;

impl ExecutionUnit for MatrixLSU {
    fn execute(instr: Instruction, state: &mut SystemState) -> Result<bool, TrapCause> {
        let mem = &mut state.mem;
        let srf = &mut state.srf;
        let mrf = &mut state.mrf;

        let base_addr = srf[instr.rs1 as usize]; 
        let stride = srf[instr.rs2 as usize] * 4; // translate stride from words to bytes
        let csr_val_base = base_addr;
        let csr_col_base = srf[instr.rs2 as usize]; // csr col

        let nnz = instr.im1 as u32;
        let md = instr.md as usize;
        let ms1 = instr.ms1 as usize;

        match instr.opcode {
            MZERO => {
                for i in 0..4 {
                    mrf[md][i] = [0; 4];
                }
            }
            MLDW => {
                for i in 0..4u32 {
                    mrf[md][i as usize] = mem.load_128b((base_addr + i*stride) as usize)?;
                }
            }
            MSTW => {
                for i in 0..4u32 {
                    mem.store_128b((base_addr + i*stride) as usize, mrf[ms1][i as usize])?;
                }
            }
            SPLDW => {
                for i in 0..4u32 {
                    if i < nnz {
                        mrf[md][0][i as usize] = mem.load_word((csr_val_base + i*4) as usize)? as i32; // csr val
                        mrf[md][1][i as usize] = mem.load_word((csr_col_base + i*4) as usize)? as i32; // csr col
                    } else {
                        mrf[md][0][i as usize] = 0;
                        mrf[md][1][i as usize] = 0;
                    }
                }
            }
            DLDW => {
                for i in 0..4u32 {
                    mrf[md][i as usize] = mem.load_128b((base_addr + (mrf[ms1][1][i as usize] as u32) * stride) as usize)?
                }
            }
            _ => unreachable!("Decoder guarantees valid instructions")
        }
        
        return Ok(false);
    }
}
