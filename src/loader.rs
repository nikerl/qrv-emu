// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use elf::{
    ElfBytes, 
    endian::AnyEndian
};

use crate::{
    system::SystemState,
    data::rf_scalar::RegNames::*
};

pub fn setup_args(args: &[String], state: &mut SystemState) {
    let mem = &mut state.mem;
    let srf = &mut state.srf;

    let argc = args.len() as u32;
    let mut str_addr_array = Vec::new();

    // Place the argument strings in a fixed scratch area near the top of memory,
    let mut addr = srf[SP] - 4096;
    for arg in args {
        str_addr_array.push(addr);
        for (i, byte) in arg.bytes().enumerate() {
            mem.store_byte((addr + i as u32) as usize, byte);
        }
        mem.store_byte((addr + arg.len() as u32) as usize, 0); // null terminator
        addr += arg.len() as u32 + 1;
    }

    // _start reads argc/argv off the stack (lw a0, 0(sp); addi a1, sp, 4), not from
    // a0/a1, so lay out [argc][argv[0]]..[argv[n-1]][NULL] well below the strings,
    // leaving the program a real stack area that can't grow back into them.
    let new_sp = (srf[SP] - 8192) & !15; // 16-byte align per the RISC-V ABI

    mem.store_word(new_sp as usize, argc);
    for (i, &str_addr) in str_addr_array.iter().enumerate() {
        mem.store_word((new_sp + 4 + i as u32 * 4) as usize, str_addr);
    }
    mem.store_word((new_sp + 4 + str_addr_array.len() as u32 * 4) as usize, 0); // argv null terminator

    srf[SP] = new_sp;
}

pub fn load_bin(path_str: String, state: &mut SystemState) {
    let mem = &mut state.mem;
    let srf = &mut state.srf;

    let path = std::path::PathBuf::from(path_str);
    let file_data = std::fs::read(path).expect("failed to read file");
    let file = ElfBytes::<AnyEndian>::minimal_parse(&file_data).expect("failed to parse ELF");

    // filter PT_LOAD program headers 
    let load_segments: Vec<_> = file.segments()
        .expect("no program headers")
        .iter()
        .filter(|p| p.p_type == elf::abi::PT_LOAD)
        .collect();

    // set the base address for automatic vaddr translation
    mem.base_addr = load_segments.iter().map(|p| p.p_vaddr as u32).min().unwrap();
    mem.program_break = load_segments.iter()
            .map(|p| (p.p_vaddr + p.p_memsz) as u32)
            .max()
            .unwrap();

    srf[PC] = file.ehdr.e_entry as usize as u32; // init pc to program's entry point
    srf[SP] = mem.len() - 0xFF; // init sp to an address just under mem length


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
