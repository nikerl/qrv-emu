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
    exec::ExecResult, 
    instruction_set::Instruction, 
    loader::{
        load_bin, 
        setup_args,
    }, system::SystemState, trap::TrapCause, 
};


fn exit_emulator(exit_status: i32) {
    println!("Exit");
    exit(exit_status)
}

fn run_emulator(state: &mut SystemState, args: Vec<String>) -> Result<(), TrapCause>  {
    load_bin(args[1].clone(), state)?;

    if args.len() > 2 {
        let guest_args = &args[2..];
        setup_args(guest_args, state)?;
    }

    loop {
        let instruction: Instruction = decode(state.mem.load_word(state.srf[PC] as usize)?)?;
        state.instruction_history.push(state.srf[PC], &instruction);
        match dispatch(instruction, state)? {
            ExecResult::Continue { branch_taken } => {
                if !branch_taken {
                    state.srf.inc_pc();
                }
            }
            ExecResult::Exit { exit_status } => exit_emulator(exit_status)
        } 
    }
}

fn main() {
    let mut state = SystemState::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Expected 1 argument: Path to binary");
        exit_emulator(1);
    }
    
    if let Err(cause) = run_emulator(&mut state, args) {
        cause.trap_debug_print(&state);
        exit_emulator(1);
    }
}
