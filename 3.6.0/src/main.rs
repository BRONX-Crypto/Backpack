mod TokenCreate;
use TokenCreate::Token;
mod Process;
mod Functions;
use Process::*;
use bitvec::prelude::*;
use std::io;
#[allow(warnings)]
fn main() {
    let mut vector: BitVec<u8, Msb0> = BitVec::new();
    let version = "3.6.0";
    let g = 3;
    println!("Backpack {} ({version})", g);
    println!("for read from terminal and play enter with zero and one");
    let mut terminal_in = String::new();
    io::stdin().read_line(&mut terminal_in).unwrap();
    let terminal_in = terminal_in.trim();
    if terminal_in.starts_with("0") || terminal_in.starts_with("1") {
    for ch in terminal_in.chars() {
    match ch {
        '0' => vector.push(false),
        '1' => vector.push(true),
        '\n' => {continue},
        _ => panic!("this character: {} it's not defined", ch),
    }
}
    }
    else {
        println!("unknown command");
    }
    let tokenize = TokenCreate::vectok(vector);
    process(tokenize.clone());
    println!("{:?}", tokenize);
}
