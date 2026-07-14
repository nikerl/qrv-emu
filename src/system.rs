// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use crate::{
    data::{
        memory::Memory, 
        rf_scalar::ScalarRF,
        rf_matrix::MatrixRF,
        filetable::FileTable
    },
};

pub struct SystemState {
    pub mem: Memory,
    pub srf: ScalarRF,
    pub mrf: MatrixRF,
    pub file_table: FileTable
}

impl SystemState {
    pub fn new() -> Self {
        return SystemState{mem: Memory::new(), srf: ScalarRF::new(), mrf: MatrixRF::new(), file_table: FileTable::new()};
    }
}
