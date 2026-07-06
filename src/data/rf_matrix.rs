use std::{fmt, ops::{Index, IndexMut}};

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
    type Output = [[i32; 4]; 4];
    fn index(&self, i: usize) -> &[[i32; 4]; 4] {
        return &self.rf[i];
    }
}
impl IndexMut<usize> for MatrixRF {
    fn index_mut(&mut self, i: usize) -> &mut [[i32; 4]; 4] {
        return &mut self.rf[i];
    }
}

impl fmt::Display for MatrixRF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..8 {
            writeln!(f, "m{}: {:?}", i, self.rf[i][0])?;
            writeln!(f, "    {:?}", self.rf[i][1])?;
            writeln!(f, "    {:?}", self.rf[i][2])?;
            writeln!(f, "    {:?}\n", self.rf[i][3])?;
        }
        Ok(())
    }
}
