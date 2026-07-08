mod decoder;
mod dispatcher;
mod data;
use data::{memory::Memory, rf_scalar::{ScalarRF, RegNames::*}, rf_matrix::MatrixRF};
mod exec;
mod instruction_set;
use instruction_set::Instruction;


fn main() {
    let mut mem = Memory::new();
    let mut rf = ScalarRF::new();
    let mut mrf = MatrixRF::new();


    rf[12] = 9; rf[13] = 10; 
    // example: ADD a11, a12, a13
    let mut instruction: Instruction = decoder::decode(0b0000_0000_1101_0110_0000_0101_1011_0011);

    println!("{:?}", instruction);

    dispatcher::dispatch(instruction, &mut rf, &mut mrf, &mut mem);

    println!("add instr result: {} + {} = {}", rf[12], rf[13], rf[11]);

    rf[1] = 42;
    rf[SP] = 0x1234_ABCD; 
    rf[PC] = 10;
    rf[SP] = 0x0000_00FF;
    println!("rf: \n{}", rf);

    mem.store_word(0x000_010c, 1337);
    println!("mem:\n{}", mem.examine(0x000_0100, 8));

    let test = mem.load_word(0x000_010c);

    println!("test: {}", test);

    mrf[2][0][3] = 420;
    println!("mrf:\n{}", mrf);

    mem.store_byte(0x0000_0010, b'H');
    mem.store_byte(0x0000_0011, b'e');
    mem.store_byte(0x0000_0012, b'l');
    mem.store_byte(0x0000_0013, b'l');
    mem.store_byte(0x0000_0014, b'o');
    mem.store_byte(0x0000_0015, b' ');
    mem.store_byte(0x0000_0016, b'W');
    mem.store_byte(0x0000_0017, b'o');
    mem.store_byte(0x0000_0018, b'r');
    mem.store_byte(0x0000_0019, b'l');
    mem.store_byte(0x0000_001a, b'd');
    mem.store_byte(0x0000_001b, b'\n');

    rf[10] = 1; 
    rf[11] = 0x0000_0010; 
    rf[12] = 12; 
    rf[17] = 64; 
    instruction = decoder::decode(0b0000_0000_0000_0000_0000_0000_0111_0011);
    dispatcher::dispatch(instruction.clone(), &mut rf, &mut mrf, &mut mem);

    rf[17] = 93; 
    rf[10] = 0; 
    dispatcher::dispatch(instruction.clone(), &mut rf, &mut mrf, &mut mem);


}
