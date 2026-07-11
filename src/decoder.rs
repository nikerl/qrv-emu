use crate::{
    instruction_set::{
        Instruction,
        InstructionSet
    },
    data::rf_scalar::RegNames::*
};

const OPCODE_MASK: u32 = 0b0000_0000_0000_0000_0000_0000_0111_1111;
const FUNC3_MASK: u32  = 0b0000_0000_0000_0000_0111_0000_0000_0000;
const FUNC7_MASK: u32  = 0b1111_1110_0000_0000_0000_0000_0000_0000;


fn sign_immediate(instruction: u32, im_start: u32, im_length: u32, signed: bool) -> i32 {

    let base: i32 = 2;
    let data_mask: u32 = (base.pow(im_length) - 1) as u32;
    
    if !signed { // unsigned, fill with zeros
        return ((instruction >> im_start) & data_mask) as i32;
    }
    else { // signed, cast to i32, shift up to set sign bit, finally shift down to correct value
        let temp = ((instruction >> im_start) as i32) << (32 - im_length);
        return temp >> (32 - im_length);
    }
}


fn parse_r_type(instruction: u32) -> Instruction {
    let func3: u8 = ((instruction & FUNC3_MASK) >> 12) as u8;
    let func7: u8 = ((instruction & FUNC7_MASK) >> 25) as u8;

    let operation: InstructionSet;

    match func3 {
        0b000 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::ADD,
                0b010_0000 => operation = InstructionSet::SUB,
                0b000_0001 => operation = InstructionSet::MUL,
                _ => panic!("Unrecognized func7")
            }
        }
        0b001 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::SLL,
                0b000_0001 => operation = InstructionSet::MULH,
                _ => panic!("Unrecognized func7")
            }
        }
        0b010 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::SLT,
                0b000_0001 => operation = InstructionSet::MULHSU,
                _ => panic!("Unrecognized func7")
            }
        }
        0b011 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::SLTU,
                0b000_0001 => operation = InstructionSet::MULHU,
                _ => panic!("Unrecognized func7")
            }
        }
        0b100 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::XOR,
                0b000_0001 => operation = InstructionSet::DIV,
                _ => panic!("Unrecognized func7")
            }
        }
        0b101 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::SRL,
                0b010_0000 => operation = InstructionSet::SRA,
                0b000_0001 => operation = InstructionSet::DIVU,
                _ => panic!("Unrecognized func7")
            }
        }
        0b110 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::OR,
                0b000_0001 => operation = InstructionSet::REM,
                _ => panic!("Unrecognized func7")
            }
        }
        0b111 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::AND,
                0b000_0001 => operation = InstructionSet::REMU,
                _ => panic!("Unrecognized func7")
            }
        }
        _ => panic!("Unrecognized func3")
    }

    let parsed_i = Instruction::new_r_type(
        operation,
        ((instruction >> 15) & 0b11111) as u8,
        ((instruction >> 20) & 0b11111) as u8,
        ((instruction >> 7) & 0b11111) as u8
    );

    return parsed_i;
}

fn parse_i_type(instruction: u32, major_opcode: u8) -> Instruction {
    let func3: u8 = ((instruction & FUNC3_MASK) >> 12) as u8;

    let operation: InstructionSet;

    // Most instructions use 12 bit immidiates
    let mut im1: i32 = sign_immediate(instruction, 20, 12, true);
    

    match major_opcode { 
        0b0001_0011 => {
            match func3 {
                0b000 => operation = InstructionSet::ADDI,
                0b001 => {
                    operation = InstructionSet::SLLI;
                    im1 = sign_immediate(instruction, 20, 5, false); // 5 bit immidiate
                }
                0b010 => operation = InstructionSet::SLTI,
                0b011 => operation = InstructionSet::SLTIU,
                0b100 => operation = InstructionSet::XORI,
                0b101 => {
                    let func7: u8 = ((instruction & FUNC7_MASK) >> 25) as u8;
                    match func7 {
                        0b000_0000 => {
                            operation = InstructionSet::SRLI;
                            im1 = sign_immediate(instruction, 20, 5, false); // 5 bit immidiate
                        }
                        0b010_0000 => {
                            operation = InstructionSet::SRAI;
                            im1 = sign_immediate(instruction, 20, 5, false); // 5 bit immidiate
                        }
                        _ => panic!("Unrecognized func7")
                    }
                }
                0b110 => operation = InstructionSet::ORI,
                0b111 => operation = InstructionSet::ANDI,
                _ => panic!("Unrecognized func3")
            }
        }
        0b0000_0011 => {
            match func3 {
                0b000 => operation = InstructionSet::LB,
                0b001 => operation = InstructionSet::LH,
                0b010 => operation = InstructionSet::LW,
                0b100 => operation = InstructionSet::LBU,
                0b101 => operation = InstructionSet::LHU,
                _ => panic!("Unrecognized func3")
            }
        }
        0b0110_0111 => operation = InstructionSet::JALR,
        0b0111_0011 => {
            im1 = sign_immediate(instruction, 20, 1, false);
            if im1 == 0 {
                operation = InstructionSet::ECALL;
            }
            else {
                operation = InstructionSet::EBREAK;
            }
        }
        0b000_1111 => operation = InstructionSet::FENCE,
        _ => panic!("Unrecognized opcode")

    }

    let parsed_i = Instruction::new_i_type(
        operation,
        im1,
        ((instruction >> 15) & 0b11111) as u8,
        ((instruction >> 7) & 0b11111) as u8
    );

    return parsed_i
}

fn parse_s_type(instruction: u32) -> Instruction {
    let func3: u8 = ((instruction & FUNC3_MASK) >> 12) as u8;

    // Connect the two immidiate fields by shifting up the lower one
    let shifted_instr = (instruction & 0xFE00_0000) | ((instruction << 13) & 0x01F0_0000);
    let im1: i32 = sign_immediate(shifted_instr, 20, 12, true);

    let operation: InstructionSet;

    match func3 {
        0b000 => operation = InstructionSet::SB,
        0b001 => operation = InstructionSet::SH,
        0b010 => operation = InstructionSet::SW,
        _ => panic!("Unrecognized func3")

    }

    let parsed_i = Instruction::new_s_type(
        operation,
        im1,
        ((instruction >> 15) & 0b11111) as u8,
        ((instruction >> 20) & 0b11111) as u8
    );

    return parsed_i
}

fn parse_b_type(instruction: u32) -> Instruction {
    let func3: u8 = ((instruction & FUNC3_MASK) >> 12) as u8;
    
    let operation: InstructionSet;

    let bit31 = (instruction >> 31) & 0b1;
    let bits30_25 = (instruction >> 25) & 0b111111;
    let bits11_8 = (instruction >> 8) & 0b1111;
    let bit7 = (instruction >> 7) & 0b1;
    let shifted_instr = (bit31 << 12) | (bit7 << 11) | (bits30_25 << 5) | (bits11_8 << 1);
    let im1: i32 =  sign_immediate(shifted_instr, 0, 13, true);

    match func3 {
        0b000 => operation = InstructionSet::BEQ,
        0b001 => operation = InstructionSet::BNE,
        0b100 => operation = InstructionSet::BLT,
        0b101 => operation = InstructionSet::BGE,
        0b110 => operation = InstructionSet::BLTU,
        0b111 => operation = InstructionSet::BGEU,
        _ => panic!("Unrecognized func3")
    }

    let parsed_i = Instruction::new_b_type(
        operation,
        im1,
        ((instruction >> 15) & 0b11111) as u8,
        ((instruction >> 20) & 0b11111) as u8
    );

    return parsed_i;
}

fn parse_u_type(instruction: u32, major_opcode: u8) -> Instruction {
    let operation: InstructionSet;

    let im1: i32 = (instruction & 0xFFFF_F000) as i32;

    match major_opcode {
        0b011_0111 => operation = InstructionSet::LUI,
        0b001_0111 => operation = InstructionSet::AUIPC,
        _ => panic!("Unrecognized opcode")
    }

    let parsed_i = Instruction::new_u_type(
        operation,
        im1,
        ((instruction >> 7) & 0b11111) as u8
    );

    return parsed_i;
}

fn parse_j_type(instruction: u32) -> Instruction {    
    let operation= InstructionSet::JAL;

    let bit31 = (instruction >> 31) & 0b1;
    let bits30_21 = (instruction >> 21) & 0b1111111111;
    let bit20 = (instruction >> 20) & 0b1;
    let bits19_12 = (instruction >> 12) & 0b11111111;
    let shifted_instr = (bit31 << 20) | (bit20 << 11) | (bits30_21 << 1) | (bits19_12 << 12);
    let im1: i32 =  sign_immediate(shifted_instr, 0, 20, true);
    
    let parsed_i = Instruction::new_j_type(
        operation, 
        im1,
        ((instruction >> 7) & 0b11111) as u8
    );

    return parsed_i;
}

fn parse_mm_type(instruction: u32) -> Instruction {
    let func7: u8 = ((instruction & FUNC7_MASK) >> 27) as u8;

    let operation: InstructionSet;
    let mut im1: i32 = 0;
    let mut rs1: u8 = 0;
    let mut rs2: u8 = 0;
    let mut ms1: u8 = 0;
    let mut ms2: u8 = 0;
    let mut md: u8 = 0;

    match func7 {
        0b11110 => {
            operation = InstructionSet::MMASAW;
            ms1 = ((instruction >> 18) & 0b111) as u8;
            ms2 = ((instruction >> 21) & 0b111) as u8;
            md = ((instruction >> 15) & 0b111) as u8
        }
        0b01000 => {
            operation = InstructionSet::SPMACW;
            ms1 = ((instruction >> 18) & 0b111) as u8;
            ms2 = ((instruction >> 21) & 0b111) as u8;
            md = ((instruction >> 15) & 0b111) as u8
        }
        0b11111 => {
            operation = InstructionSet::MZERO;
            md = ((instruction >> 15) & 0b111) as u8
        }
        0b00000 => {
            operation = InstructionSet::MLDW;
            rs2 = ((instruction >> 20) & 0b11111) as u8;
            rs1 = ((instruction >> 15) & 0b11111) as u8;
            md = ((instruction >> 7) & 0b111) as u8
        }
        0b00001 => {
            operation = InstructionSet::MSTW;
            rs2 = ((instruction >> 20) & 0b11111) as u8;
            rs1 = ((instruction >> 15) & 0b11111) as u8;
            ms1 = ((instruction >> 7) & 0b111) as u8
        }
        0b00100 => {
            operation = InstructionSet::SPLDW;
            im1 = ((instruction >> 15) & 0b111) as i32;
            rs2 = MS2 as u8; // col base
            rs1 = MS1 as u8; // val base
            md = ((instruction >> 7) & 0b111) as u8
        }
        0b00010 => {
            operation = InstructionSet::DLDW;
            rs2 = MS2 as u8; // base
            rs1 = MS1 as u8; // stride
            ms1 = ((instruction >> 15) & 0b111) as u8;
            md = ((instruction >> 7) & 0b111) as u8
        }
        _ => panic!("Unrecognized func7")
    }

    let parsed_i = Instruction::new_mm_type(operation, im1, rs1, rs2, ms1, ms2, md);

    return parsed_i;
}

pub fn decode(instruction: u32) -> Instruction {
    let major_opcode = (instruction & OPCODE_MASK) as u8;
    match major_opcode {
        0b011_0011 => return parse_r_type(instruction),
        0b001_0011 | 0b0000_0011 | 0b0110_0111 | 0b0111_0011 | 0b000_1111 => return parse_i_type(instruction, major_opcode),
        0b010_0011 => return parse_s_type(instruction),
        0b110_0011 => return parse_b_type(instruction),
        0b110_1111 => return parse_j_type(instruction),
        0b011_0111 | 0b001_0111 => return parse_u_type(instruction, major_opcode),
        0b010_1011 => return parse_mm_type(instruction),
        _ => panic!("Unrecognized opcode") // fix more graceful exception
    }
}
