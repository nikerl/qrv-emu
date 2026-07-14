// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use std::process::exit;

use crate::{
    system::SystemState,
    instruction_set::{
        InstrType, 
        Instruction,
        InstructionSet::*
    }, 
    exec::{
        ExecutionUnit,
        alu::Alu,
        branch::Branch,
        jump::Jump,
        lsu::Lsu,
        sys::Sys,
        matrix_lsu::MatrixLSU,
        matrix_multiply::MatrixMultiply
    }
};


/// Returns bool: branch taken (true), or not taken (false)
pub fn dispatch(instruction: Instruction, state: &mut SystemState) -> bool {
    match instruction.instr_type {
        InstrType::RType => {
            return Alu::execute(instruction, state);
        }
        InstrType::IType => {
            match instruction.opcode {
                ADDI | XORI | ORI | ANDI | SLLI | SRLI | SRAI | SLTI | SLTIU => return Alu::execute(instruction, state),
                LB | LH | LW | LBU | LHU => return Lsu::execute(instruction, state),
                JALR => return Jump::execute(instruction, state),
                ECALL | EBREAK | FENCE => return Sys::execute(instruction, state),
                _ => {println!("Unrecognized opcode"); exit(1)}
            }
        }
        InstrType::SType => {
            return Lsu::execute(instruction, state);
        }
        InstrType::BType => {
            return Branch::execute(instruction, state);
        }
        InstrType::UType => {
            return Alu::execute(instruction, state);
        }
        InstrType::JType => {
            return Jump::execute(instruction, state);
        }
        InstrType::MMType => {
            match instruction.opcode {
                MMASAW | SPMACW => return MatrixMultiply::execute(instruction, state),
                MZERO | MLDW | MSTW | SPLDW | DLDW => return MatrixLSU::execute(instruction, state),
                _ => {println!("Unrecognized opcode"); exit(1)}
            }
        }
    }
}
