use std::process::exit;

use crate::{
    data::{
        memory::Memory, 
        rf_matrix::MatrixRF, 
        rf_scalar::ScalarRF
    }, 
    instruction_set::{
        InstrType, 
        Instruction,
        InstructionSet::*
    }, 
    exec::{
        ScalarFU, 
        MatrixFU,
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
pub fn dispatch(instruction: Instruction, srf: &mut ScalarRF, mrf: &mut MatrixRF, mem: &mut Memory) -> bool {
    match instruction.instr_type {
        InstrType::RType => {
            return Alu::execute(instruction, srf, mem);
        }
        InstrType::IType => {
            match instruction.opcode {
                ADDI | XORI | ORI | ANDI | SLLI | SRLI | SRAI | SLTI | SLTIU => return Alu::execute(instruction, srf, mem),
                LB | LH | LW | LBU | LHU => return Lsu::execute(instruction, srf, mem),
                JALR => return Jump::execute(instruction, srf, mem),
                ECALL | EBREAK | FENCE => return Sys::execute(instruction, srf, mem),
                _ => {println!("Unrecognized opcode"); exit(1)}
            }
        }
        InstrType::SType => {
            return Lsu::execute(instruction, srf, mem);
        }
        InstrType::BType => {
            return Branch::execute(instruction, srf, mem);
        }
        InstrType::UType => {
            return Alu::execute(instruction, srf, mem);
        }
        InstrType::JType => {
            return Jump::execute(instruction, srf, mem);
        }
        InstrType::MMType => {
            match instruction.opcode {
                MMASAW | SPMACW => return MatrixMultiply::execute(instruction, srf, mrf, mem),
                MZERO | MLDW | MSTW | SPLDW | DLDW => return MatrixLSU::execute(instruction, srf, mrf, mem),
                _ => {println!("Unrecognized opcode"); exit(1)}
            }
        }
    }
}
