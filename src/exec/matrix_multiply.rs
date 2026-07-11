use crate::{
    data::{
        memory::Memory, 
        rf_scalar::ScalarRF,
        rf_matrix::MatrixRF
    }, 
    instruction_set::{
        Instruction, 
        InstructionSet::*
    }, 
    exec::MatrixFU
};


pub struct MatrixMultiply;

impl MatrixFU for MatrixMultiply {
    fn execute(instr: Instruction, _srf: &mut ScalarRF, mrf: &mut MatrixRF, _mem: &mut Memory) -> bool {
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
            _ => println!("Unrecognized opcode")
        }

        return false;
    }
}
