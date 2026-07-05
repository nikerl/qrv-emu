use std::{fmt, ops::{Index, IndexMut}};

#[derive(Debug)]
pub struct RegisterFile {
    rf: [u32; 32],
    pub pc: u32,
}

impl RegisterFile {
    pub fn new() -> Self {
        return RegisterFile {rf: [0; 32], pc: 0};
    }

    pub fn zero(&self) -> u32 {
        return self.rf[0];
    }
    pub fn ra(&self) -> u32 {
        return self.rf[1];
    }
    pub fn set_ra(&mut self, new_ra: u32) {
        self.rf[1] = new_ra;
    }
    pub fn sp(&self) -> u32 {
        return self.rf[2];
    }
    pub fn set_sp(&mut self, new_ra: u32) {
        self.rf[2] = new_ra;
    }
    pub fn gp(&self) -> u32 {
        return self.rf[3];
    }
    pub fn set_gp(&mut self, new_ra: u32) {
        self.rf[3] = new_ra;
    }
    pub fn inc_pc(&mut self) {
        self.pc += 4;
    }
}

impl Index<usize> for RegisterFile {
    type Output = u32;
    fn index(&self, i: usize) -> &u32 {
        &self.rf[i]
    }
}
impl IndexMut<usize> for RegisterFile {
    fn index_mut(&mut self, i: usize) -> &mut u32 {
        &mut self.rf[i]
    }
}

impl fmt::Display for RegisterFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..32 {
            writeln!(f, "x{}: {}", i, self.rf[i])?;
        }
        writeln!(f, "pc: {}", self.pc)?;
        Ok(())
    }
}
