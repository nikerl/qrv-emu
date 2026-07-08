#[derive(Debug)]
#[derive(Clone)]
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
    BGE, //branch if $≥$
    BLTU, //branch if $<$ (u)
    BGEU, //branch if $≥$ (u)

    // Jump instructions
    JAL, //jump and link
    JALR, //jump and link reg

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
#[derive(Clone)]
pub enum InstrType {
    RType,
    IType,
    SType,
    BType,
    UType,
    JType,
    MMType,
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Instruction {
    pub instr_type: InstrType,
    pub opcode: InstructionSet,
    pub im1: i32,
    pub im2: i32,
    pub rs1: u8,
    pub rs2: u8,
    pub rd: u8,
    pub ms1: u8,
    pub ms2: u8,
    pub md: u8
}
impl Instruction {
    pub fn new_r_type(opcode: InstructionSet, rs1: u8, rs2: u8, rd: u8) -> Self {
        Instruction { instr_type: InstrType::RType, opcode, im1: 0, im2: 0, rs1, rs2, rd, ms1: 0, ms2: 0, md: 0 }
    }
    pub fn new_i_type(opcode: InstructionSet, im1: i32, rs1: u8, rd: u8) -> Self {
        Instruction { instr_type: InstrType::IType, opcode, im1, im2: 0, rs1, rs2: 0, rd, ms1: 0, ms2: 0, md: 0 }
    }
    pub fn new_s_type(opcode: InstructionSet, im1: i32, rs1: u8, rs2: u8) -> Self {
        Instruction { instr_type: InstrType::SType, opcode, im1, im2: 0, rs1, rs2, rd: 0, ms1: 0, ms2: 0, md: 0 }
    }
    pub fn new_b_type(opcode: InstructionSet, im1: i32, rs1: u8, rs2: u8) -> Self {
        Instruction { instr_type: InstrType::BType, opcode, im1, im2: 0, rs1, rs2, rd: 0, ms1: 0, ms2: 0, md: 0 }
    }
    pub fn new_u_type(opcode: InstructionSet, im1: i32, rd: u8) -> Self {
        Instruction { instr_type: InstrType::UType, opcode, im1, im2: 0, rs1: 0, ms2: 0, rd, rs2: 0, ms1: 0, md: 0}
    }
    pub fn new_j_type(opcode: InstructionSet, im1: i32, rd: u8) -> Self {
        Instruction { instr_type: InstrType::JType, opcode, im1, im2: 0, rs1: 0, ms2: 0, rd, rs2: 0, ms1: 0, md: 0}
    }
    pub fn new_mm_type(opcode: InstructionSet, im1: i32, rs1: u8, rs2: u8, ms1: u8, ms2: u8, md: u8) -> Self {
        Instruction { instr_type: InstrType::MMType, opcode, im1, im2: 0, rs1, rs2, rd: 0, ms1, ms2, md}
    }
}
