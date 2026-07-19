// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

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
#[derive(PartialEq)]
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
    pub md: u8,
    pub raw_i: u32
}
impl Instruction {
    pub fn new_r_type(opcode: InstructionSet, rs1: u8, rs2: u8, rd: u8, raw_i: u32) -> Self {
        Instruction { instr_type: InstrType::RType, opcode, im1: 0, im2: 0, rs1, rs2, rd, ms1: 0, ms2: 0, md: 0, raw_i }
    }
    pub fn new_i_type(opcode: InstructionSet, im1: i32, rs1: u8, rd: u8, raw_i: u32) -> Self {
        Instruction { instr_type: InstrType::IType, opcode, im1, im2: 0, rs1, rs2: 0, rd, ms1: 0, ms2: 0, md: 0, raw_i }
    }
    pub fn new_s_type(opcode: InstructionSet, im1: i32, rs1: u8, rs2: u8, raw_i: u32) -> Self {
        Instruction { instr_type: InstrType::SType, opcode, im1, im2: 0, rs1, rs2, rd: 0, ms1: 0, ms2: 0, md: 0, raw_i }
    }
    pub fn new_b_type(opcode: InstructionSet, im1: i32, rs1: u8, rs2: u8, raw_i: u32) -> Self {
        Instruction { instr_type: InstrType::BType, opcode, im1, im2: 0, rs1, rs2, rd: 0, ms1: 0, ms2: 0, md: 0, raw_i }
    }
    pub fn new_u_type(opcode: InstructionSet, im1: i32, rd: u8, raw_i: u32) -> Self {
        Instruction { instr_type: InstrType::UType, opcode, im1, im2: 0, rs1: 0, ms2: 0, rd, rs2: 0, ms1: 0, md: 0, raw_i}
    }
    pub fn new_j_type(opcode: InstructionSet, im1: i32, rd: u8, raw_i: u32) -> Self {
        Instruction { instr_type: InstrType::JType, opcode, im1, im2: 0, rs1: 0, ms2: 0, rd, rs2: 0, ms1: 0, md: 0, raw_i}
    }
    pub fn new_mm_type(opcode: InstructionSet, im1: i32, rs1: u8, rs2: u8, ms1: u8, ms2: u8, md: u8, raw_i: u32) -> Self {
        Instruction { instr_type: InstrType::MMType, opcode, im1, im2: 0, rs1, rs2, rd: 0, ms1, ms2, md, raw_i}
    }
}
impl Default for Instruction {
    fn default() -> Self {
        Instruction { instr_type: InstrType::IType, opcode: InstructionSet::ADDI, im1: 0, im2: 0, rs1: 0, rs2: 0, rd: 0, ms1: 0, ms2: 0, md: 0, raw_i: 0 }
    }
}

/// Circular buffer for storing the most recently decoded instructions.
/// 
/// The buffer is a fixed size vector of tuples of (PC, Instruction).
/// 
/// When the buffer fills up it will overwrite the oldest entries.
pub struct InstrBuffer {
    buffer: Vec<(u32, Instruction)>,
    write_ptr: usize,
    size: usize,
}
impl InstrBuffer {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Size should be greater than zero")
        }
        InstrBuffer { buffer: vec![(0, Instruction::default()); size], write_ptr: 0, size }
    }
    pub fn push(&mut self, pc: u32, instruction: &Instruction) {
        self.buffer[self.write_ptr] = (pc, instruction.clone());
        self.write_ptr = (self.write_ptr + 1) % self.size;
    }

    pub fn dump(&self) -> String {
        let mut output: String = format!("{:<10}   {:<6}   {:<34}\n", "PC", "Opcode", "Raw Instruction Word");
        for i in 0..self.size {
            let index = (self.write_ptr + i) % self.size;
            output.push_str(&format!("0x{:08x} | {:<6} | 0b{:032b}\n", self.buffer[index].0, format!("{:?}", self.buffer[index].1.opcode), self.buffer[index].1.raw_i));
        }

        return output;
    }
}
