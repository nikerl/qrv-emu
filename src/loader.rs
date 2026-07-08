use elf::{
    ElfBytes, 
    endian::AnyEndian
};

use crate::{
    data::{
        memory::Memory, 
        rf_scalar::{ScalarRF, RegNames::*}
    }
};

pub fn load_bin(path_str: String, srf: &mut ScalarRF, mem: &mut Memory) {
    let path = std::path::PathBuf::from(path_str);
    let file_data = std::fs::read(path).expect("failed to read file");
    let file = ElfBytes::<AnyEndian>::minimal_parse(&file_data).expect("failed to parse ELF");

    // filter PT_LOAD program headers 
    let load_segments: Vec<_> = file.segments()
        .expect("no program headers")
        .iter()
        .filter(|p| p.p_type == elf::abi::PT_LOAD)
        .collect();

    srf[PC] = file.ehdr.e_entry as u32; // init pc to program's entry point
    srf[SP] = mem.len() - 0xFF; // init sp to an address just under mem length

    // set the base address for automatic vaddr translation
    mem.base_addr = load_segments.iter().map(|p| p.p_vaddr as u32).min().unwrap();
    mem.program_break = load_segments.iter()
        .map(|p| (p.p_vaddr + p.p_memsz) as u32)
        .max().unwrap();


    for phdr in &load_segments  {
        let vaddr = phdr.p_vaddr as usize;
        let filesz = phdr.p_filesz as usize;
        let offset = phdr.p_offset as usize;

        // copy file bytes into memory at vaddr
        for i in 0..filesz {
            mem.store_byte(vaddr + i, file_data[offset + i]);
        }
    }
}
