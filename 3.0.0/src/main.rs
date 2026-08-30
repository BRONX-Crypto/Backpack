mod TokenCreate;
use TokenCreate::Token;
mod Process;
use Process::*;
use bitvec::prelude::*;
fn main() {
    let vector = bitvec![u8, Msb0; 0, 0, 1, 1, 1];
    let tokenize = TokenCreate::vectok(vector);
    process(tokenize.clone());
    println!("{:?}", tokenize);
}
