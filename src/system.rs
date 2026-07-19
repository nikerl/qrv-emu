// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use crate::{
    data::{
        filetable::FileTable, 
        memory::Memory, 
        rf_matrix::MatrixRF, 
        rf_scalar::ScalarRF
    }, 
    instruction_set::InstrBuffer
};

pub struct SystemState {
    pub mem: Memory,
    pub srf: ScalarRF,
    pub mrf: MatrixRF,
    pub file_table: FileTable,
    pub instruction_history: InstrBuffer
}

impl SystemState {
    pub fn new() -> Self {
        return SystemState{mem: Memory::new(), srf: ScalarRF::new(), mrf: MatrixRF::new(), file_table: FileTable::new(), instruction_history: InstrBuffer::new(16)};
    }
}
