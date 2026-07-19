// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use crate::{
    instruction_set::Instruction, 
    system::SystemState, 
    trap::TrapCause
};

pub mod alu;
pub mod jump;
pub mod branch;
pub mod lsu;
pub mod matrix_lsu;
pub mod matrix_multiply;
pub mod sys;


pub enum ExecResult{
    Continue { branch_taken: bool },
    Exit { exit_status: i32 }
}

/// Execute returns a bool: true if branch is taken, false if not taken. To skip PC increment after branch
/// 
/// Only applicable for branch and jump type instructions, all others should return false
pub trait ExecutionUnit {
    fn execute(instr: Instruction, state: &mut SystemState) -> Result<ExecResult, TrapCause>;
}
