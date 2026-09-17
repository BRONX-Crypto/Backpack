#[allow(warnings)]
mod Lexer;
mod Process;
use Lexer::opcode;
mod Functions;
use bitvec::prelude::*;
use std::io;
use Lexer::*;
use Lexer::Lex;
use Process::*;
use Lexer::opcode::*;
#[allow(warnings)]
fn main() {
    let mut vector: BitVec<u8, Msb0> = bitvec![u8, Msb0; 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1];
    let version = "4.3.0";
    let g = 4;
    println!("Backpack {} ({version})", g);
    println!("{}", vector);
    let var = Lex(vector);
    println!("{:?}", var);
    parse(var);
}
