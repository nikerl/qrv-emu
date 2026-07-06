use crate::{
    data::{
        memory::Memory, 
        rf_matrix::MatrixRF, 
        rf_scalar::ScalarRF
    }, 
    decoder::{
        InstrType, 
        Instruction,
        InstructionSet
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
                InstructionSet::ADDI | InstructionSet::XORI | InstructionSet::ORI 
                    | InstructionSet::ANDI | InstructionSet::SLLI | InstructionSet::SRLI 
                    | InstructionSet::SRAI | InstructionSet::SLTI | InstructionSet::SLTIU => Alu::execute(instruction, srf, mem),
                InstructionSet::LB | InstructionSet::LH | InstructionSet::LW 
                    | InstructionSet::LBU | InstructionSet::LHU => Lsu::execute(instruction, srf, mem),
                InstructionSet::JALR => Jump::execute(instruction, srf, mem),
                InstructionSet::ECALL | InstructionSet::EBREAK | InstructionSet::FENCE => Sys::execute(instruction, srf, mem),
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
                InstructionSet::MMASAW | InstructionSet::SPMACW => MatrixMultiply::execute(instruction, srf, mrf, mem),
                InstructionSet::MZERO | InstructionSet::MLDW | InstructionSet::MSTW 
                    | InstructionSet::SPLDW | InstructionSet::DLDW => MatrixLSU::execute(instruction, srf, mrf, mem),
                _ => println!("Unrecognized opcode")
            }
        }
    }
}
