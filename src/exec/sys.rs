// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use std::{
    fs::{
        File,
        OpenOptions
    }, io::{
        self, Read, Write
    }, process::exit, time::SystemTime
};

use crate::{
    data::rf_scalar::RegNames::*,
    instruction_set::{
        Instruction, 
        InstructionSet::*
    }, 
    exec::ExecutionUnit,
    system::SystemState,
    trap::TrapCause
};

pub struct Sys;

impl ExecutionUnit for Sys {
    fn execute(instr: Instruction, state: &mut SystemState) -> Result<bool, TrapCause> {
        let mem = &mut state.mem;
        let regs = &mut state.srf;
        let file_table = &mut state.file_table;

        match instr.opcode {
            ECALL => {
                let syscall_num = regs[A7];
                match syscall_num {
                    57 => { // close
                        let fd = regs[A0] as i32;
                        regs[A0] = file_table.remove(fd) as u32;
                    }

                    62 => { // lseek
                        let fd = regs[A0] as i32;
                        let offset = regs[A1] as i32;
                        let whence = regs[A2];

                        match file_table.files.get_mut(&fd) {
                            Some(file) => {
                                use std::io::{Seek, SeekFrom};
                                let seek_from = match whence {
                                    0 => SeekFrom::Start(offset as u64),   // SEEK_SET
                                    1 => SeekFrom::Current(offset as i64), // SEEK_CUR
                                    2 => SeekFrom::End(offset as i64),     // SEEK_END
                                    _ => { regs[A0] = -1i32 as u32; return Ok(false); }
                                };
                                match file.seek(seek_from) {
                                    Ok(pos) => regs[A0] = pos as u32,
                                    Err(_) => regs[A0] = -1i32 as u32,
                                }
                            }
                            _ => regs[A0] = -1i32 as u32,
                        }
                    }

                    63 => { // read
                        let fd = regs[A0];
                        let str_ptr = regs[A1];
                        let str_len = regs[A2];
                        let mut buf = vec![0u8; str_len as usize];
                        
                        let bytes_read = if fd == 0 {
                            io::stdin().read(&mut buf)
                        }
                        else {
                            match file_table.files.get_mut(&(fd as i32)) {
                                Some(file) => file.read(&mut buf),
                                None => { regs[A0] = -1i32 as u32; return Ok(false); } // adjust to your actual control flow
                            }
                        };
                        
                        match bytes_read {
                            Ok(n) => {
                                for i in 0..n {
                                    mem.store_byte((str_ptr as usize) + i, buf[i])?;
                                }
                                regs[A0] = n as u32;
                            }
                            Err(_) => regs[A0] = -1i32 as u32,
                        }
                    }

                    64 => { // write
                        let fd = regs[A0];
                        let str_ptr = regs[A1];
                        let str_len = regs[12];

                        let mut bytes: Vec<u8> = Vec::with_capacity(str_len as usize);
                        for i in 0..str_len {
                            bytes.push(mem.load_byte((str_ptr + i) as usize)?);
                        }

                        if fd == 1 { // stdout
                            io::stdout().write_all(&bytes).expect("Can't write to stdout");
                        }
                        else if fd == 2 { // stderr
                            io::stderr().write_all(&bytes).expect("Can't write to stderr");
                        }
                        else { // file
                            let f = &mut file_table.files.get(&(fd as i32)).unwrap();
                            f.write_all(&bytes).expect("Can't write to fd");
                        }
                        regs[A0] = str_len;
                    }

                    80 => { // fstat
                        let buf_ptr = regs[A1] as usize;
                        for i in 0..88 {
                            mem.store_byte(buf_ptr + i, 0)?;
                        }
                        let s_ifchr: u32 = 0o020000;
                        mem.store_word(buf_ptr + 4, s_ifchr)?; // st_mode
                        regs[A0] = 0;
                    }

                    93 => { // program exit
                        println!("Exit");
                        exit(regs[A0] as i32);
                    }

                    214 => { // brk, increase heap limit if possible
                        let new_break = regs[A0];
                        if (new_break > mem.program_break) && (new_break < regs[SP]) {
                            mem.program_break = new_break;
                        }
                        regs[A0] = mem.program_break;
                    }

                    403 => { // clock_gettime
                        let clk_id = regs[A0];
                        let tp_addr = regs[A1] as usize;

                        if clk_id == 0 || clk_id == 1 { // Realtime
                            let time = SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap();

                            mem.store_word(tp_addr, time.as_secs() as u32)?;
                            mem.store_word(tp_addr + 4, time.subsec_nanos() as u32)?;

                            regs[A0] = 0;
                        }
                        else { // Unrecognized clk_id
                            regs[A0] = -1i32 as u32;
                        }
                    }

                    1024 => { // open
                        let file_path: String = mem.load_str(regs[A0] as usize)?;
                        let flags: u32 = regs[A1];

                        let is_write = (flags & 0x3) != 0;
                        let create = (flags & 0x200) != 0;
                        let truncate = (flags & 0x400) != 0;

                        let open_file = if is_write {
                            OpenOptions::new()
                                .write(true)
                                .create(create)
                                .truncate(truncate)
                                .open(&file_path)
                        } else {
                            File::open(&file_path)
                        };
                        
                        if open_file.is_err() {
                            println!("cant open file\n {:?}", open_file);
                            regs[A0] = -1i32 as u32;
                        }
                        else {
                            regs[A0] = file_table.insert(open_file.unwrap()) as u32;
                        }
                    }
                    _ => return Err(TrapCause::UnhandledSyscall{ num: syscall_num })
                } 
            }
            EBREAK => println!("EBREAK hit at pc {:#x}", regs[PC]),
            FENCE => return Ok(false), // not relevant 
            _ => unreachable!("Decoder guarantees valid instructions")
        }

        return Ok(false);
    }
}
