use std::{fmt, ops::{Index, IndexMut}};

const MEM_SIZE: usize = 0x000_FFFF; // Memory size in bytes

pub struct Memory {
    mem: [u32; MEM_SIZE],
    mem_size: u32,
}

impl Memory {
    pub fn new() -> Self{
        return Memory { mem: [0; MEM_SIZE], mem_size: MEM_SIZE as u32 }
    }

    pub fn examine(self, location: u32, num_words: u32) -> String{
        let mut output: String = "".to_owned();
        for i in 0..num_words {
            output.push_str(
                &format!("0x{:08x}: 0x{:02x}{:02x}{:02x}{:02x}\n", 
                    i*4 + location, 
                    self.mem[(i*4 + 3 + location) as usize], 
                    self.mem[(i*4 + 2 + location) as usize], 
                    self.mem[(i*4 + 1 + location) as usize], 
                    self.mem[(i*4 + location) as usize]
                )
            );
        }
        return output;
    }
}

impl Index<usize> for Memory {
    type Output = u32;
    fn index(&self, i: usize) -> &u32 {
        &self.mem[i]
    }
}
impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, i: usize) -> &mut u32 {
        &mut self.mem[i]
    }
}
