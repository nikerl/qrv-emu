
mod decoder;

fn main() {

    // example: ADD a11, a12, a13
    let instruction: decoder::Instruction = decoder::decode(0b0000_0000_1101_0110_0000_0101_1011_0011);

    println!("instruction fetched: {:#?}", instruction);
}
