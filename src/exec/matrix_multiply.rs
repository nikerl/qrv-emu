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

pub struct MatrixMultiply;

impl ExecutionUnit for MatrixMultiply {
    fn execute(instr: Instruction, state: &mut SystemState) -> Result<bool, TrapCause> {
        let mrf = &mut state.mrf;

        let ms1 = instr.ms1 as usize;
        let ms2 = instr.ms2 as usize;
        let md = instr.md as usize; 

        match instr.opcode {
            MMASAW => {
                for i in 0..4 {
                    for j in 0..4 {
                        for k in 0..4 {
                            mrf[md][i][j] += mrf[ms1][i][k] * mrf[ms2][j][k]; // ms2 transposed
                        }
                    }
                }
            }
            SPMACW => {
                for i in 0..4 {
                    for j in 0..4 {
                        mrf[md][0][j] += mrf[ms1][0][i] * mrf[ms2][i][j];
                    }
                }
            }
            _ => unreachable!("Decoder guarantees valid instructions")
        }

        return Ok(false);
    }
}
