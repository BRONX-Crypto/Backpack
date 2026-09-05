#[allow(warnings)]
mod TokenCreate;
use TokenCreate::Token;
mod Process;
mod Functions;
use Process::*;
use bitvec::prelude::*;
use std::io;
use make_colors::*;
use std::path::*;
use std::fs::*;
use std::fs;
use std::io::stdout;
use std::io::Write;
fn main() {
    let mut vector: BitVec<u8, Msb0> = BitVec::new();
    let version = vec![10, 0];
    let g = 3;
    println!("Backpack {} ({g}.{}.{})", g, version[0], version[1]);
    println!("for read from terminal and play it, enter with zero and one");
    print!("for read from File From Path and play it Starts with (x)");
    io::stdout().flush().unwrap();
    let mut terminal_in = String::new();
    io::stdin().read_line(&mut terminal_in).unwrap();
    let mut terminal_in = terminal_in.trim();
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
    if terminal_in.starts_with("x") {
        let replaced = terminal_in.replace("x", "");
        let p = &replaced;
        let path = Path::new(p);
        if path.exists() {
            let mut content: Vec<u8> = fs::read(path).unwrap();
            let mut t: BitVec<u8, Msb0> = BitVec::from_vec(content);
            println!("enter not real bits number: ");
            let mut i = String::new();
            io::stdin().read_line(&mut i).unwrap();
            let i: u8 = i.trim().parse().unwrap();
            let find_value = t.len() - i as usize;
            t.truncate(find_value);
            for x in t {
                vector.push(x);
            }

        }
        if !path.exists() {
            println!("not Find This File, maybe This File it's not exists on that path");
        }

    }
    else {
        println!("{}", make_colors_rgb("undefined command", (255, 0, 0), None));
    }
    let tokenize = TokenCreate::vectok(vector);
    process(tokenize.clone());
    println!("{:?}", tokenize);
}
