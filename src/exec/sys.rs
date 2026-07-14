use std::{fs::{File, OpenOptions}, io::{self, Read}, process::exit};

use crate::{
    data::{
        memory::Memory, 
        rf_scalar::{ScalarRF, RegNames::*}
    }, 
    instruction_set::{
        Instruction, 
        InstructionSet::*
    }, 
    exec::ScalarFU
};

pub struct Sys;

impl ScalarFU for Sys {
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory) -> bool {
        match instr.opcode {
            ECALL => {
                let syscall_num = regs[A7];
                match syscall_num {
                    57 => { // close, not needed just return success
                        regs[A0] = 0; 
                    }
                    64 => { // write to stdout
                        let fd = regs[A0];
                        let str_ptr = regs[A1];
                        let str_len = regs[12];
                        for i in 0..str_len {
                            if fd == 1 {
                                print!("{}", mem.load_byte((str_ptr + i) as usize) as char);
                            }
                            else if fd == 2 {
                                eprint!("{}", mem.load_byte((str_ptr + i) as usize) as char);
                            }
                        }
                        regs[A0] = str_len;
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
                            match mem.files.get_mut(&(fd as i32)) {
                                Some(file) => file.read(&mut buf),
                                None => { regs[A0] = -1i32 as u32; return false; } // adjust to your actual control flow
                            }
                        };

                        match bytes_read {
                            Ok(n) => {
                                for i in 0..n {
                                    mem.store_byte((str_ptr as usize) + i, buf[i]);
                                }
                                regs[A0] = n as u32;
                            }
                            Err(_) => regs[A0] = -1i32 as u32,
                        }
                    }
                    80 => { // fstat: zero the struct, mark as char device (tty-like), return 0
                        let buf_ptr = regs[A1] as usize;
                        for i in 0..88 {
                            mem.store_byte(buf_ptr + i, 0);
                        }
                        let s_ifchr: u32 = 0o020000;
                        mem.store_word(buf_ptr + 4, s_ifchr); // st_mode
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
                    1024 => { // open
                        let file_path: String = mem.load_str(regs[A0] as usize);
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
                            let mut fd = 1;
                            while mem.files.contains_key(&fd) {
                                fd += 1;
                            }
                            mem.files.insert(fd, open_file.unwrap());
    
                            regs[A0] = fd as u32;
                        }
                    }
                    _ => println!("Unhandled syscall: {}", syscall_num)
                } 
            }
            EBREAK => println!("EBREAK hit at pc {:#x}", regs[PC]),
            FENCE => return false, // not relevant 
            _ => println!("Unrecognized opcode")
        }

        return false;
    }
}
