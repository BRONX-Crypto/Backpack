mod TokenCreate;
use TokenCreate::Token;
mod Process;
mod Functions;
use Process::*;
use bitvec::prelude::*;
use std::io;
fn main() {
    let mut vector: BitVec<u8, Msb0> = BitVec::new();
    let version = "3.4.0";
    println!("Backpack 3 ({version})");
    println!("for read from terminal and play enter with zero and one");
    let mut terminal_in = String::new();
    io::stdin().read_line(&mut terminal_in).unwrap();
    let terminal_in = terminal_in.trim();
    for ch in terminal_in.chars() {
    match ch {
        '0' => vector.push(false),
        '1' => vector.push(true),
        '\n' => {continue},
        _ => panic!("this character: {} it's not defined", ch),
    }
}
    let tokenize = TokenCreate::vectok(vector);
    process(tokenize.clone());
    println!("{:?}", tokenize);
}
