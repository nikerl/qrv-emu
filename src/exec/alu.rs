use crate::{data::{memory::Memory, rf_scalar::ScalarRF}, instruction_set::Instruction, exec::ScalarFU};

pub struct Alu;

// ADD, SUB, XOR, OR, AND, SLL, SRL, SRA, SLT, SLTU, MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU, ADDI, XORI, ORI, ANDI, SLLI, SRLI, SRAI, SLTI, SLTIU, LUI, AUIPC
impl ScalarFU for Alu {
    fn execute(instr: Instruction, regs: &mut ScalarRF, mem: &mut Memory) {
        println!("instruction dispatched to alu: {:#?}", instr);
        //todo!()
    }
}
