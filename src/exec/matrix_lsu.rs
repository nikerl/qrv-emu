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


pub struct MatrixLSU;

impl MatrixFU for MatrixLSU {
    fn execute(instr: Instruction, srf: &mut ScalarRF, mrf: &mut MatrixRF, mem: &mut Memory) -> bool {
        let base_addr = srf[instr.rs1 as usize]; // csr val
        let stride = srf[instr.rs2 as usize]; // csr col
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
                    mrf[md][i as usize] = mem.load_128b((base_addr + i*stride) as usize);
                }
            }
            MSTW => {
                for i in 0..4u32 {
                    mem.store_128b((base_addr + i*stride) as usize, mrf[ms1][i as usize]);
                }
            }
            SPLDW => {
                mrf[md][0] = mem.load_128b(base_addr as usize); // csr val
                mrf[md][1] = mem.load_128b(stride as usize); // csr col
            }
            DLDW => {
                for i in 0..4u32 {
                    mrf[md][i as usize] = mem.load_128b((base_addr + (mrf[md][1][i as usize] as u32) * stride) as usize)
                }
            }
            _ => println!("Unrecognized opcode")
        }
        
        return false;
    }
}
