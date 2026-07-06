
mod decoder;
mod data;
use data::{rf_scalar::ScalarRF, rf_matrix::MatrixRF};


fn main() {
    let mut rf = ScalarRF::new();
    let mut mrf = MatrixRF::new();

    // example: ADD a11, a12, a13
    let instruction: decoder::Instruction = decoder::decode(0b0000_0000_1101_0110_0000_0101_1011_0011);

    println!("instruction fetched: {:#?}", instruction);

    rf[1] = 42;
    rf.set_sp(0x1234_ABCD); 
    rf.pc = 10;
    println!("rf: \n{}", rf);

    mrf[2][0][3] = 420;
    println!("mrf:\n{}", mrf);

}
