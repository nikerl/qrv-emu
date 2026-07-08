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
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory) {
        match instr.opcode {
            ECALL => {
                let syscall_num = regs[17];
                match syscall_num {
                    93 => { // program exit
                        println!("Exit");
                        exit(regs[10] as i32);
                    }
                    64 => { // write to stdout
                        let fd = regs[10];
                        let str_ptr = regs[11];
                        let str_len = regs[12];
                        for i in 0..str_len {
                            if fd == 1 {
                                print!("{}", mem.load_byte((str_ptr + i) as usize) as char);
                            }
                            else if fd == 2 {
                                eprint!("{}", mem.load_byte((str_ptr + i) as usize) as char);
                            }
                        }
                        regs[10] = str_len;
                    }
                    63 => { // read from stdin
                        let str_ptr = regs[11];
                        let str_len = regs[12];
                        let mut buf = vec![0u8; str_len as usize];
                        if io::stdin().read_exact(&mut buf).is_err() {
                            println!("Error reading stdin");
                            regs[10] = 0;
                        }
                        else {
                            for i in 0..str_len {
                                mem.store_byte((str_ptr + i) as usize, buf[i as usize]);
                            }
                            regs[10] = str_len;
                        }
                        
                    }
                    214 => { // brk, increase heap limit if possible
                        let new_break = regs[10];
                        if (new_break > mem.program_break) && (new_break < regs[SP]) {
                            mem.program_break = new_break;
                        }
                        regs[10] = mem.program_break;
                    }
                    _ => println!("Unhandled syscall: {}", syscall_num)
                } 
            }
            EBREAK => println!("EBREAK hit at pc {:#x}", regs[PC]),
            FENCE => return, // not relevant 
            _ => println!("Unrecognized opcode")
        }
    }
}
