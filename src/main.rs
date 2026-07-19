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
mod trap;

use std::{env, process::exit};

use crate::{
    data::rf_scalar::RegNames::*, 
    decoder::decode, 
    dispatcher::dispatch, 
    instruction_set::Instruction, 
    loader::{
        load_bin, 
        setup_args,
    }, 
    system::SystemState, 
    trap::TrapCause, 
};


fn run_emulator(state: &mut SystemState, args: Vec<String>) -> Result<(), TrapCause>  {
    load_bin(args[1].clone(), state)?;

    if args.len() > 2 {
        let guest_args = &args[2..];
        setup_args(guest_args, state)?;
    }

    loop {
        let instruction: Instruction = decode(state.mem.load_word(state.srf[PC] as usize)?)?;
        state.instruction_history.push(state.srf[PC], &instruction);
        let branch_taken: bool = dispatch(instruction, state)?;

        if !branch_taken {
            state.srf.inc_pc();
        }
    }
}

fn main() {
    let mut state = SystemState::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Expected 1 argument: Path to binary");
        exit(1);
    }
    
    if let Err(cause) = run_emulator(&mut state, args) {
        cause.trap_handler(&state);
    }
}
