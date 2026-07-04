#[derive(Debug)]
pub enum InstructionSet {
    // R-type arithmetic
    ADD, //add
    SUB, //subtract
    XOR, //bitwise exclusive or
    OR, //bitwise or
    AND, //bitwise and
    SLL, //shift left logical
    SRL, //shift right logical
    SRA, //shift right arith
    SLT, //set less than
    SLTU, //set less than (u)

    // I-type arithmetic
    ADDI, //add immediate
    XORI, //xor immediate
    ORI, //or immediate
    ANDI, //and immediate
    SLLI, //shift left logical imm
    SRLI, //shift right logical imm
    SRAI, //shift right arith imm
    SLTI, //set less than imm
    SLTIU, //set less than imm (u)

    // instructions
    LI, //load immediate
    LA, //load address

    MV, //move (copy)
    NEG, //2s-complement negation
    NOT, //bitwise not

    // Load instructions
    LB, //load byte
    LH, //load half
    LW, //load word
    LBU, //load byte (u)
    LHU, //load half (u)

    // Store instructions
    SB, //store byte
    SH, //store half
    SW, //store word

    // Branch instructions
    BEQ, //branch if $=$
    BNE, //branch if $≠$
    BLT, //branch if $<$
    BLE, //branch if $≤$
    BGT, //branch if $>$
    BGE, //branch if $≥$
    BLTU, //branch if $<$ (u)
    BLEU, //branch if $≤$ (u)
    BGTU, //branch if $>$ (u)
    BGEU, //branch if $≥$ (u)

    // Branch zero instructions
    BEQZ, //branch if $= 0$
    BNEZ, //branch if $≠ 0$
    BLTZ, //branch if $< 0$
    BLEZ, //branch if $≤ 0$
    BGTZ, //branch if $> 0$
    BGEZ, //branch if $≥ 0$

    // Jump instructions
    JAL, //jump and link
    JALR, //jump and link reg
    J, //jump
    CALL, //call subroutine
    RET, //return

    // Upper immediate
    LUI, //load upper imm
    AUIPC, //add upper imm to pc

    // System instructions
    ECALL, //environment call
    EBREAK, //environment break
    FENCE, //fence
    
    // Multiply extension
    MUL, //multiply
    MULH, //multiply high signed
    MULHSU, //multiply high signed x unsigned
    MULHU, //multiply high unsigned
    DIV, //divide signed
    DIVU, //divide unsigned
    REM, //remainder signed
    REMU, //remainder unsigned

    // X-HEEP Matrix Spec Integer
    MMASAW, //matrix multiply accumulate
    MZERO, //zero matrix register
    MLDW, //load matrix tile
    MSTW, //store matrix tile

    // QuadriSparse
    SPLDW, //sparse tile load
    DLDW, //dense tile load
    SPMACW, //sparse matrix multiply with Gustavson
}

#[derive(Debug)]
pub enum InstrType {
    RType,
    IType,
    SType,
    BType,
    UType,
    JType,
}


const OPCODE_MASK: u32 = 0b0000_0000_0000_0000_0000_0000_0111_1111;
const FUNC3_MASK: u32  = 0b0000_0000_0000_0000_0111_0000_0000_0000;
const FUNC7_MASK: u32  = 0b1111_1110_0000_0000_0000_0000_0000_0000;


#[derive(Debug)]
pub struct Instruction {
    instr_type: InstrType,
    opcode: InstructionSet,
    im1: i32,
    im2: i32,
    rs1: u8,
    rs2: u8,
    rd: u8,
}
impl Instruction {
    fn new_r_type(opcode: InstructionSet, rs1: u8, rs2: u8, rd: u8) -> Self {
        Instruction { instr_type: InstrType::RType, opcode, im1: 0, im2: 0, rs1, rs2, rd }
    }
    fn new_i_type(opcode: InstructionSet, im1: i32, rs1: u8, rd: u8) -> Self {
        Instruction { instr_type: InstrType::IType, opcode, im1, im2: 0, rs1, rs2: 0, rd }
    }
    fn new_sb_type(opcode: InstructionSet, im1: i32, im2: i32, rs1: u8, rs2: u8) -> Self {
        Instruction { instr_type: InstrType::SType, opcode, im1, im2, rs1, rs2, rd: 0 }
    }
    fn new_uj_type(opcode: InstructionSet, im1: i32, rd: u8) -> Self {
        Instruction { instr_type: InstrType::SType, opcode, im1, im2: 0, rs1: 0, rs2: 0, rd}
    }
}

fn sign_immediate(instruction: u32, im_length: u32, signed: bool) -> i32 {

    let base: i32 = 2;
    let data_mask: u32 = (base.pow(im_length) - 1) as u32;
    
    if !signed { // unsigned, fill with zeros
        return ((instruction >> 20) & data_mask) as i32;
    }
    else { // signed, cast to i32, shift up to set sign bit, finally shift down to correct value
        let temp = ((instruction >> 20) as i32) << (32 - im_length);
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
                _ => panic!("Unrecognized func7") // fix more graceful exception
            }
        }
        0b001 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::SLL,
                0b000_0001 => operation = InstructionSet::MULH,
                _ => panic!("Unrecognized func7") // fix more graceful exception
            }
        }
        0b010 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::SLT,
                0b000_0001 => operation = InstructionSet::MULHSU,
                _ => panic!("Unrecognized func7") // fix more graceful exception
            }
        }
        0b011 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::SLTU,
                0b000_0001 => operation = InstructionSet::MULHU,
                _ => panic!("Unrecognized func7") // fix more graceful exception
            }
        }
        0b100 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::XOR,
                0b000_0001 => operation = InstructionSet::DIV,
                _ => panic!("Unrecognized func7") // fix more graceful exception
            }
        }
        0b101 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::SRL,
                0b010_0000 => operation = InstructionSet::SRA,
                0b000_0001 => operation = InstructionSet::DIVU,
                _ => panic!("Unrecognized func7") // fix more graceful exception
            }
        }
        0b110 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::OR,
                0b000_0001 => operation = InstructionSet::REM,
                _ => panic!("Unrecognized func7") // fix more graceful exception
            }
        }
        0b111 => {
            match func7 {
                0b000_0000 => operation = InstructionSet::AND,
                0b000_0001 => operation = InstructionSet::REMU,
                _ => panic!("Unrecognized func7") // fix more graceful exception
            }
        }
        _ => panic!("Unrecognized func3") // fix more graceful exception
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
    let mut im1: i32 = sign_immediate(instruction, 12, true);
    

    match major_opcode { 
        0b0001_0011 => {
            match func3 {
                0b000 => operation = InstructionSet::ADDI,
                0b001 => {
                    operation = InstructionSet::SLLI;
                    im1 = sign_immediate(instruction, 4, false); // 4 bit immidiate
                }
                0b010 => operation = InstructionSet::SLTI,
                0b011 => operation = InstructionSet::SLTIU,
                0b100 => operation = InstructionSet::XORI,
                0b101 => {
                    let func7: u8 = ((instruction & FUNC7_MASK) >> 25) as u8;
                    match func7 {
                        0b000_0000 => {
                            operation = InstructionSet::SRLI;
                            im1 = sign_immediate(instruction, 4, false); // 4 bit immidiate
                        }
                        0b010_0000 => {
                            operation = InstructionSet::SRAI;
                            im1 = sign_immediate(instruction, 4, false); // 4 bit immidiate
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
            im1 = sign_immediate(instruction, 1, false);
            if im1 == 0 {
                operation = InstructionSet::ECALL;
            }
            else {
                operation = InstructionSet::EBREAK;
            }
        }
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
    todo!()
}
fn parse_b_type(instruction: u32) -> Instruction {
    todo!()
}
fn parse_u_type(instruction: u32) -> Instruction {
    todo!()
}
fn parse_j_type(instruction: u32) -> Instruction {
    todo!()
}

pub fn decode(instruction: u32) -> Instruction {
    let major_opcode = (instruction & OPCODE_MASK) as u8;
    match major_opcode {
        0b0011_0011 => return parse_r_type(instruction),
        0b0001_0011 | 0b0000_0011 | 0b0110_0111 | 0b0111_0011 => return parse_i_type(instruction, major_opcode),
        0b0010_0011 => return parse_s_type(instruction),
        0b0110_0011 => return parse_b_type(instruction),
        0b0110_1111 => return parse_j_type(instruction),
        0b0011_0111 => return parse_u_type(instruction),
        _ => panic!("Unrecognized opcode") // fix more graceful exception
    }
}
