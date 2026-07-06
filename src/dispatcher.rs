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

pub fn dispatch(instruction: Instruction, srf: &mut ScalarRF, mrf: &mut MatrixRF, mem: &mut Memory) {
    match instruction.instr_type {
        InstrType::RType => { // all to alu
            Alu::execute(instruction, srf, mem);
        }
        InstrType::IType => {
            match instruction.opcode {
                ADDI | XORI | ORI | ANDI | SLLI | SRLI | SRAI | SLTI | SLTIU => Alu::execute(instruction, srf, mem),
                LB | LH | LW | LBU | LHU => Lsu::execute(instruction, srf, mem),
                JALR => Jump::execute(instruction, srf, mem),
                ECALL | EBREAK | FENCE => Sys::execute(instruction, srf, mem),
                _ => println!("Unrecognized opcode")
            }
        }
        InstrType::SType => { // all to lsu
            Lsu::execute(instruction, srf, mem);
        }
        InstrType::BType => { // all to branch
            Branch::execute(instruction, srf, mem);
        }
        InstrType::UType => {
            Alu::execute(instruction, srf, mem);
        }
        InstrType::JType => { // only JAL
            Jump::execute(instruction, srf, mem);
        }
        InstrType::MMType => {
            match instruction.opcode {
                MMASAW | SPMACW => MatrixMultiply::execute(instruction, srf, mrf, mem),
                MZERO | MLDW | MSTW | SPLDW | DLDW => MatrixLSU::execute(instruction, srf, mrf, mem),
                _ => println!("Unrecognized opcode")
            }
        }
    }
}
