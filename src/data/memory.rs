use std::{ops::{Index, IndexMut}};

const MEM_SIZE: usize = 0x0000_FFFF; // Memory size in bytes

pub struct Memory {
    mem: [u8; MEM_SIZE],
    pub program_break: u32
}

impl Memory {
    pub fn new() -> Self{
        return Memory { mem: [0; MEM_SIZE], program_break: 0 }
    }

    pub fn load_word(&self, addr: usize) -> u32 {
        return (self.mem[addr + 3] as u32) << 24 | (self.mem[addr + 2] as u32) << 16 | (self.mem[addr + 1] as u32) << 8 | self.mem[addr] as u32;
    }
    pub fn store_word(&mut self, addr: usize, word: u32) {
        self.mem[addr] = word as u8;
        self.mem[addr + 1] = (word >> 8) as u8;
        self.mem[addr + 2] = (word >> 16) as u8;
        self.mem[addr + 3] = (word >> 24) as u8;
    }
    pub fn load_half(&self, addr: usize) -> u16 {
        return (self.mem[addr + 1] as u16) << 8 | self.mem[addr] as u16;
    }
    pub fn store_half(&mut self, addr: usize, half: u16) {
        self.mem[addr] = half as u8;
        self.mem[addr + 1] = (half >> 8) as u8;
    }
    pub fn load_byte(&self, addr: usize) -> u8 {
        return self.mem[addr] as u8;
    }
    pub fn store_byte(&mut self, addr: usize, byte: u8) {
        self.mem[addr] = byte as u8;
    }

    pub fn len(&self) -> u32 {
        return MEM_SIZE as u32;
    }

    pub fn examine(&self, location: u32, num_words: u32) -> String{
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
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        &self.mem[i]
    }
}
impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        &mut self.mem[i]
    }
}
