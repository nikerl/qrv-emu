mod decoder;
mod dispatcher;
mod data;
use data::{memory::Memory, rf_scalar::ScalarRF, rf_matrix::MatrixRF};
mod exec;
mod instruction_set;
use instruction_set::Instruction;



fn main() {
    let mut mem = Memory::new();
    let mut rf = ScalarRF::new();
    let mut mrf = MatrixRF::new();


    rf[12] = 9; rf[13] = 10; 
    // example: ADD a11, a12, a13
    let instruction: Instruction = decoder::decode(0b0000_0000_1101_0110_0000_0101_1011_0011);

    println!("{:?}", instruction);

    dispatcher::dispatch(instruction, &mut rf, &mut mrf, &mut mem);

    println!("add instr result: {} + {} = {}", rf[12], rf[13], rf[11]);

    rf[1] = 42;
    rf.set_sp(0x1234_ABCD); 
    rf.pc = 10;
    println!("rf: \n{}", rf);

    mem[0x000_010c] = 1337;
    println!("mem:\n{}", mem.examine(0x000_0100, 8));

    mrf[2][0][3] = 420;
    println!("mrf:\n{}", mrf);

}
