use std::{ops::{Index, IndexMut}, process::exit};

const MEM_SIZE: usize = 0x004F_FFFF; // Memory size in bytes

pub const TOHOST_ADDR: usize = 0x1000_0000;
pub const  IO_WRITE_ADDR: usize = 0x1000_1000;
pub const  RVMODEL_HALT_PASS: u32 = 1;
pub const  RVMODEL_HALT_FAIL: u32 = 3;

pub struct Memory {
    mem: Vec<u8>,
    pub program_break: u32,
    pub base_addr: u32,
}

impl Memory {
    pub fn new() -> Self{
        return Memory { mem: vec![0; MEM_SIZE as usize], program_break: 0, base_addr: 0 }
    }

    fn to_host(&self, word: u32) {
        if word == RVMODEL_HALT_PASS {
            println!("Test PASSED\nExiting...");
            exit(0);
        }
        else if word == RVMODEL_HALT_FAIL {
            println!("Test FAILED\nExiting...");
            exit(1);
        }
    }

    pub fn translate_vaddr(&self, vaddr: usize) -> usize {
        let addr: usize = ((vaddr as u32) - self.base_addr) as usize;
        if addr >= MEM_SIZE as usize {
            panic!("Memory access out of bounds: vaddr {:#x} (translated {:#x})", vaddr, addr);
        }
        return addr;
    }

    pub fn load_word(&self, addr: usize) -> u32 {
        let addr = self.translate_vaddr(addr);
        return (self.mem[addr + 3] as u32) << 24 | (self.mem[addr + 2] as u32) << 16 | (self.mem[addr + 1] as u32) << 8 | self.mem[addr] as u32;
    }
    pub fn store_word(&mut self, addr: usize, word: u32) {
        if addr == TOHOST_ADDR { // Intercept writes to host
            self.to_host(word);
            return;
        }
        else if addr == IO_WRITE_ADDR { // Intercepts IO writes
            print!("{}", word as u8 as char);
            return;
        }

        let addr = self.translate_vaddr(addr);
        self.mem[addr] = word as u8;
        self.mem[addr + 1] = (word >> 8) as u8;
        self.mem[addr + 2] = (word >> 16) as u8;
        self.mem[addr + 3] = (word >> 24) as u8;
    }
    pub fn load_half(&self, addr: usize) -> u16 {
        let addr = self.translate_vaddr(addr);
        return (self.mem[addr + 1] as u16) << 8 | self.mem[addr] as u16;
    }
    pub fn store_half(&mut self, addr: usize, half: u16) {
        let addr = self.translate_vaddr(addr);
        self.mem[addr] = half as u8;
        self.mem[addr + 1] = (half >> 8) as u8;
    }
    pub fn load_byte(&self, addr: usize) -> u8 {
        let addr = self.translate_vaddr(addr);
        return self.mem[addr] as u8;
    }
    pub fn store_byte(&mut self, addr: usize, byte: u8) {
        let addr = self.translate_vaddr(addr);
        self.mem[addr] = byte as u8;
    }

    pub fn len(&self) -> u32 {
        return MEM_SIZE as u32;
    }

    #[allow(dead_code)]
    pub fn examine(&self, location: u32, num_words: u32) -> String{
        let location: u32 = self.translate_vaddr(location as usize) as u32;
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
        let i = self.translate_vaddr(i);
        &self.mem[i]
    }
}
impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        let i = self.translate_vaddr(i);
        &mut self.mem[i]
    }
}
