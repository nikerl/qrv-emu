// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

mod loader;
mod decoder;
mod dispatcher;
mod instruction_set;
mod data;
mod exec;
mod system;
use std::env;

use crate::{
    data::rf_scalar::RegNames::*,
    instruction_set::Instruction, 
    system::SystemState,
    loader::{
        load_bin, 
        setup_args,
    },
    decoder::decode, 
    dispatcher::dispatch, 
};

fn main() {
    let mut state = SystemState::new();

    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        load_bin(args[1].clone(), &mut state);

        if args.len() > 2 {
            let guest_args = &args[2..];
            setup_args(guest_args, &mut state);
        }
    }
    else {
        println!("Error: Expected 1 argument: Path to binary");
        return;
    }

    loop {
        let instruction: Instruction = decode(state.mem.load_word(state.srf[PC] as usize));
        let branch_taken: bool = dispatch(instruction, &mut state);

        if !branch_taken {
            state.srf.inc_pc();
        }
    }
}
