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

    load_bin("path_str".to_string(), &mut srf, &mut mem);

    loop {
        let instruction: Instruction = decode(mem.load_word(srf[PC] as usize));
        dispatch(instruction, &mut srf, &mut mrf, &mut mem);
    }
}
