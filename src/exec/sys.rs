use std::{io::{self, Read}, process::exit};

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
                    63 => { // read from stdin
                        let str_ptr = regs[A1];
                        let str_len = regs[A2];
                        let mut buf = vec![0u8; str_len as usize];
                        if io::stdin().read_exact(&mut buf).is_err() {
                            println!("Error reading stdin");
                            regs[A0] = 0;
                        }
                        else {
                            for i in 0..str_len {
                                mem.store_byte((str_ptr + i) as usize, buf[i as usize]);
                            }
                            regs[A0] = str_len;
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
