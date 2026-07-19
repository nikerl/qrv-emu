// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use std::fmt;

use crate::{
    data::rf_scalar::RegNames::PC, 
    system::SystemState
};

pub enum TrapCause {
    IllegalInstruction { raw_i: u32 },
    MisalignedLoad { vaddr: usize},
    MisalignedStore { vaddr: usize},
    LoadAccessFault { vaddr: usize, real_addr: usize },
    StoreAccessFault { vaddr: usize, real_addr: usize },
    UnhandledSyscall { num: u32 },
}

impl TrapCause {
    pub fn trap_handler(self, state: &SystemState) {
        eprintln!("==============TRAP===============");
        eprintln!("Exception: {:?}", self);
        eprintln!("PC: 0x{:08x}", state.srf[PC]);
        eprint!("MTVAL: ");
        match self { // mtval
            TrapCause::IllegalInstruction { raw_i } => eprintln!("bad instruction word: {:#32b}\n", raw_i),
            TrapCause::MisalignedLoad { vaddr } => eprintln!("misaligned load at {:#x}\n", vaddr),
            TrapCause::MisalignedStore { vaddr } => eprintln!("misaligned store at {:#x}\n", vaddr),
            TrapCause::LoadAccessFault { vaddr, real_addr } => eprintln!("load fault: vaddr {:#x}, real {:#x}\n", vaddr, real_addr),
            TrapCause::StoreAccessFault { vaddr, real_addr } => eprintln!("store fault: vaddr {:#x}, real {:#x}\n", vaddr, real_addr),
            TrapCause::UnhandledSyscall { num } => eprintln!("unhandled syscall {}\n", num),
        }
        eprintln!("============TRACEBACK============\n{}", state.instruction_history.dump());

        eprintln!("========SCALAR REGISTERS=========\n{}", state.srf);

        eprintln!("========MATRIX REGISTERS=========\n{}", state.mrf);

        std::process::exit(1);
    }
}

impl fmt::Debug for TrapCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TrapCause::IllegalInstruction { .. } => "IllegalInstruction",
            TrapCause::MisalignedLoad { .. } => "MisalignedLoad",
            TrapCause::MisalignedStore { .. } => "MisalignedStore",
            TrapCause::LoadAccessFault { .. } => "LoadAccessFault",
            TrapCause::StoreAccessFault { .. } => "StoreAccessFault",
            TrapCause::UnhandledSyscall { .. } => "UnhandledSyscall",
        };
        write!(f, "{}", name)
    }
}
