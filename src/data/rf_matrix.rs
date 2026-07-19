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

const MATRIX_SIZE: usize = 4;

#[derive(Debug)]
pub struct MatrixRF {
    rf: [[[i32; MATRIX_SIZE]; MATRIX_SIZE]; 8],
}

impl MatrixRF {
    pub fn new() -> Self {
        return MatrixRF {rf: [[[0; MATRIX_SIZE]; MATRIX_SIZE]; 8]};
    }
}

impl Index<usize> for MatrixRF {
    type Output = [[i32; MATRIX_SIZE]; MATRIX_SIZE];
    fn index(&self, i: usize) -> &[[i32; MATRIX_SIZE]; MATRIX_SIZE] {
        return &self.rf[i];
    }
}
impl IndexMut<usize> for MatrixRF {
    fn index_mut(&mut self, i: usize) -> &mut [[i32; MATRIX_SIZE]; MATRIX_SIZE] {
        return &mut self.rf[i];
    }
}

impl fmt::Display for MatrixRF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            writeln!(f, "m{}: {:<3?}        m{}: {:<3?}", i, self.rf[i][0], i+4, self.rf[i+4][0])?;
            writeln!(f, "    {:<3?}            {:<3?}", self.rf[i][1], self.rf[i+4][1])?;
            writeln!(f, "    {:<3?}            {:<3?}", self.rf[i][2], self.rf[i+4][2])?;
            writeln!(f, "    {:<3?}            {:<3?}\n", self.rf[i][3], self.rf[i+4][3])?;
        }
        Ok(())
    }
}
