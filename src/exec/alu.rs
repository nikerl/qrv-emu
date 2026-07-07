use crate::{
    data::{
        memory::Memory, 
        rf_scalar::ScalarRF
    }, 
    instruction_set::{
        Instruction, 
        InstructionSet::*
    }, 
    exec::ScalarFU
};

pub struct Alu;

impl ScalarFU for Alu {
    fn execute(instr: Instruction, regs: &mut ScalarRF, _mem: &mut Memory) {
        println!("instruction dispatched to alu: {:#?}", instr);

        let im1 = instr.im1;
        let rs1 = instr.rs1 as usize;
        let rs2 = instr.rs2 as usize;
        let rd = instr.rd as usize;
        
        match instr.opcode {
            ADD => regs[rd] = regs[rs1].wrapping_add(regs[rs2]),
            SUB => regs[rd] = regs[rs1].wrapping_sub(regs[rs2]),
            XOR => regs[rd] = regs[rs1] ^ regs[rs2],
            OR => regs[rd] = regs[rs1] | regs[rs2],
            AND => regs[rd] = regs[rs1] & regs[rs2],
            SLL => regs[rd] = regs[rs1] << (regs[rs2] & 0b11111),
            SRL => regs[rd] = regs[rs1] >> (regs[rs2] & 0b11111),
            SRA => regs[rd] = ((regs[rs1] as i32) >> (regs[rs2] & 0b11111)) as u32,
            SLT => if (regs[rs1] as i32) < (regs[rs2] as i32) {regs[rd] = 1} else {regs[rd] = 0}
            SLTU => if regs[rs1] < regs[rs2] {regs[rd] = 1} else {regs[rd] = 0}
            MUL => regs[rd] = regs[rs1].wrapping_mul(regs[rs2]),
            MULH => {
                let result = (regs[rs1] as i32 as i64) * (regs[rs2] as i32 as i64);
                regs[rd] = (result >> 32) as u32;
            }
            MULHSU => {
                let result = (regs[rs1] as i32 as i64) * (regs[rs2] as i64); // rs2 zero filled
                regs[rd] = (result >> 32) as u32;
            }
            MULHU => {
                let result = (regs[rs1] as u64) * (regs[rs2] as u64);
                regs[rd] = (result >> 32) as u32;
            }
            DIV => {
                if regs[rs2] == 0 {
                    regs[rd] = u32::MAX; // division by zero
                } else {
                    regs[rd] = (regs[rs1] as i32).wrapping_div(regs[rs2] as i32) as u32;
                }
            }
            DIVU => {
                if regs[rs2] == 0 {
                    regs[rd] = u32::MAX; // division by zero
                } else {
                    regs[rd] = regs[rs1].wrapping_div(regs[rs2]);
                }
            }
            REM => {
                if regs[rs2] == 0 {
                    regs[rd] = regs[rs1]; // division by zero
                } else {
                    regs[rd] = ((regs[rs1] as i32).wrapping_rem(regs[rs2] as i32)) as u32;
                }
            }
            REMU => {
                if regs[rs2] == 0 {
                    regs[rd] = regs[rs1]; // division by zero
                } else {
                    regs[rd] = regs[rs1].wrapping_rem(regs[rs2]);
                }
            }
            ADDI => regs[rd] = regs[rs1].wrapping_add(im1 as u32),
            XORI => regs[rd] = regs[rs1] ^ im1 as u32,
            ORI => regs[rd] = regs[rs1] | im1 as u32,
            ANDI => regs[rd] = regs[rs1] & im1 as u32,
            SLLI => regs[rd] = regs[rs1] << im1 as u32,
            SRLI => regs[rd] = regs[rs1] >> im1 as u32,
            SRAI => regs[rd] = ((regs[rs1] as i32) >> im1) as u32,
            SLTI => if (regs[rs1] as i32) < im1 {regs[rd] = 1} else {regs[rd] = 0},
            SLTIU => if regs[rs1] < (im1 as u32) {regs[rd] = 1} else {regs[rd] = 0},
            LUI => regs[rd] = im1 as u32,
            AUIPC => regs[rd] = regs.pc.wrapping_add(im1 as u32),
            _ => println!("Unrecognized opcode")
        }
    }
}
