// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use std::{
    fmt, 
    ops::{
        Index, 
        IndexMut
    }
};
use RegNames::*;

#[allow(dead_code)]
pub enum RegNames {
    ZERO = 0,
    RA = 1,
    SP = 2,
    GP = 3,
    TP = 4,
    A0 = 10,
    A1 = 11,
    A2 = 12,
    A3 = 13,
    A4 = 14,
    A5 = 15,
    A6 = 16,
    A7 = 17,
    MS1 = 20,
    MS2 = 21,
    PC = 32,
}


#[derive(Debug)]
pub struct ScalarRF {
    rf: [u32; 32],
    pc: u32,
    /// Discard target for writes to x0. IndexMut must return &mut u32 without
    /// a Result, so writes to x0 are redirected here instead of being applied.
    /// Never read from directly, Index<x0> always returns a literal &0.
    x0_sink: u32,
}

impl ScalarRF {
    pub fn new() -> Self {
        return ScalarRF {rf: [0; 32], pc: 0, x0_sink: 0};
    }
    
    pub fn inc_pc(&mut self) {
        self.pc += 4;
    }
}

impl Index<usize> for ScalarRF {
    type Output = u32;
    fn index(&self, i: usize) -> &u32 {
        if i == ZERO as usize {
            return &0
        }
        else if i == PC as usize {
            return &self.pc;
        }
        else {
            return &self.rf[i];
        }
    }
}
impl IndexMut<usize> for ScalarRF {
    fn index_mut(&mut self, i: usize) -> &mut u32 {
        if i == ZERO as usize {
            return &mut self.x0_sink
        }
        else if i == PC as usize {
            return &mut self.pc;
        }
        else {
            return &mut self.rf[i];
        }
    }
}
impl Index<RegNames> for ScalarRF {
    type Output = u32;
    fn index(&self, r: RegNames) -> &u32 {
        &self[r as usize] // reuses existing Index<usize> impl
    }
}
impl IndexMut<RegNames> for ScalarRF {
    fn index_mut(&mut self, r: RegNames) -> &mut u32 {
        &mut self[r as usize] // reuses existing Index<usize> impl
    }
}

impl fmt::Display for ScalarRF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..16 {
            writeln!(f, "x{:<2}: 0x{:08x}        x{}: 0x{:08x}", i, self.rf[i], i+16, self.rf[i+16])?;
        }
        writeln!(f, "pc: 0x{:08x}", self.pc)?;
        Ok(())
    }
}
