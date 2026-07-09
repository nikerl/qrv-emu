mod loader;
mod decoder;
mod dispatcher;
mod instruction_set;
mod data;
mod exec;

use crate::{
    data::{
        memory::Memory, 
        rf_scalar::{
            ScalarRF, 
            RegNames::*
        }, 
        rf_matrix::MatrixRF
    },
    instruction_set::Instruction,
    loader::load_bin,
    decoder::decode,
    dispatcher::dispatch,
};

fn main() {
    let mut mem = Memory::new();
    let mut srf = ScalarRF::new();
    let mut mrf = MatrixRF::new();

    load_bin("hello_world".to_string(), &mut srf, &mut mem);

    loop {
        let instruction: Instruction = decode(mem.load_word(srf[PC] as usize));
        //println!("{:?}\n{:#b}\n{:#x}", instruction, mem.load_word(srf[PC] as usize), srf[PC]);
        let branch_taken: bool = dispatch(instruction, &mut srf, &mut mrf, &mut mem);
        //println!("reg 10: {}, reg 14: {}, sp: {}", srf[10], srf[14], srf[SP]);

        if !branch_taken {
            srf.inc_pc();
        }
    }
}
