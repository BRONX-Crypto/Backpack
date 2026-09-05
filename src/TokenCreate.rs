#[allow(warnings)]
use bitvec::prelude::*;
use crate::Functions::*;
use make_colors::*;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    nop,
    push(BitVec<u8, Msb0>),
    pop,
    plus,
    minus,
    swap,
    copy,
    compare,
    Do(u64),
    Do_IF(u64),
    obo(mode),
    Done,
    Duplicate_Select(u64),
    swap_Select(u64, u64),
    swap_select_to_last(u64),
    call(u64),
    ret,
    pop_select_fs(popmode),
    clear(clearmodes)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum mode {
    XOR,
    AND,
    OR,
    NOT,
} use mode::*;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum clearmodes {
    onStack,
    onHeap,
} use clearmodes::*;
#[derive(Eq, Debug, PartialEq, Clone)]
pub enum popmode {
    FromStack(u64),
    FromInLine(u64),
} use popmode::*;
//vectok = vector to token
pub fn vectok(vector: BitVec<u8, Msb0>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut bss: BitVec<u8, Msb0> = BitVec::new();
    while i < vector.len() {
        if !vector[i] && !vector[i + 1] && !vector[i + 2] && !vector[i+3] && !vector[i+4] {
            tokens.push(Token::nop);
            i += 5;
            continue;
        }
        if !vector[i] && !vector[i+1] && !vector[i+2] && !vector[i+3] && vector[i+4] {
            i += 5;
            let (data, _i) = read_to_vec(&vector, &bss, i);
            i = _i;
        tokens.push(Token::push(data));
        continue;
        }
        if vector[i] == false && vector[i+1] == false && vector[i+2] == false && vector[i+3] == true && vector[i+4] == false {
            tokens.push(Token::pop);
            i += 5;
            continue;
        }
        if vector[i] == false && vector[i+1] == false && vector[i+2] == false && vector[i+3] == true && vector[i+4] == true {
            tokens.push(Token::plus);
            i += 5;
            continue;
        }
        if vector[i] == false && vector[i+1] == false && vector[i+2] == true && vector[i+3] == false && vector[i+4] == false {
            tokens.push(Token::minus);
            i += 5;
            continue;
        }
        if vector[i] == false && vector[i+1] == false && vector[i+2] == true && vector[i+3] == false && vector[i+4] == true {
            tokens.push(Token::swap);
            i += 5;
            continue;
        }
        if vector[i] == false && vector[i+1] == false && vector[i+2] == true && vector [i+3] == true && vector[i+4] == true {
            tokens.push(Token::copy);
            i += 5;
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == false && vector[i+3] == false && vector[i+4] == false {
            tokens.push(Token::compare);
            i += 5;
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == false && vector[i+3] == false && vector[i+4] == true{
            i += 5;
            let (data, _i) = read_to_u64(&vector, &bss, i);
            tokens.push(Token::Do(data));
            i = _i;
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == false && vector[i+3] == true && vector[i+4] == false {
            i += 5;
            let (ipnumber, _i) = read_to_u64(&vector, &bss, i);
            tokens.push(Token::Do_IF(ipnumber));
            i = _i;
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == false && vector[i+3] == true && vector[i+4] == true {
            i += 5;
            let result = match (vector[i], vector[i+1]) {
                (false, false) => XOR,
                (false, true) => AND,
                (true, false) => OR,
                (true, true) => NOT,
                _ => todo!(),
            };
            tokens.push(Token::obo(result));
            i += 2;
            continue;
        }
        if !vector[i] && vector[i+1] && vector[i+2] && !vector[i+3] && !vector[i+4] {
            tokens.push(Token::Done);
            i += 5;
            continue;
        }
        if !vector[i] && vector[i+1] && vector[i+2] && !vector[i+3] && vector[i+4] {
            i += 5;
            let (data, _i) = read_to_u64(&vector, &bss, i);
            i = _i;
            tokens.push(Token::Duplicate_Select(data));
            continue
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == true && vector[i+3] == true && vector[i+4] == false {
            i += 5;
            let (fd, i0) = read_to_u64(&vector, &bss, i);
            let (sd, i1) = read_to_u64(&vector, &bss, i0);
            i = i1;
            tokens.push(Token::swap_Select(fd, sd));
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == true && vector[i+3] == true && vector[i+4] == true {
            i += 5;
            let (data, _i) = read_to_u64(&vector, &bss, i);
            tokens.push(Token::swap_select_to_last(data));
            i = _i;
            continue;
        }
        if vector[i] == true && vector[i+1] == false && vector[i+2] == false && vector[i+3] == false && vector[i+4] == false {
            i += 5;
            let (data, _i) = read_to_u64(&vector, &bss, i);
            tokens.push(Token::call(data));
            i = _i;
            continue;
        }
        if vector[i] == true && vector[i+1] == false && vector[i+2] == false && vector[i+3] == false && vector[i+4] == true {
            i += 5;
            tokens.push(Token::ret);
            continue;
        }
        if vector[i] == true && vector[i+1] == false && vector[i+2] == false && vector[i+3] == true && vector[i+4] == false {
            i += 5;
            if vector[i] == false {
                i += 1;
                let (takeLast, _i) = read_to_u64(&vector, &bss, i);
                tokens.push(Token::pop_select_fs(FromInLine(takeLast)));
                i = _i;
            }
            else if vector[i] == true {
                i += 1;
                let (takeLast, _i) = read_to_u64(&vector, &bss, i);
                i = _i;
                tokens.push(Token::pop_select_fs(FromStack(takeLast)));
            }
            continue;

        }
        if vector[i] == true && vector[i+1] == false && vector[i+2] == false && vector[i+3] == true && vector[i+4] == true {
            i += 5;
            if vector[i] == false {
                tokens.push(Token::clear(onStack));
            }
            else if vector[i] == true {
                tokens.push(Token::clear(onHeap));
            }
            i += 1;
            continue;
        }
        if vector[i] == true && vector[i+1] == false && vector[i+2] == true && vector[i+3] == false && vector[i+4] == false {
            i += 5;
            let mut start = i;
            i += 4;
            let v = &vector[start..=i];
            bss.clear();
            for x in v {
                bss.push(*x);
            }
            i += 1;
            continue;
        }
        else {
            print!("{}", make_colors_rgb("Lexer:", (255, 0, 0), None));
            print!("{}", make_colors_rgb(" This binary data not matches with any opcode", (255, 0, 0), None));
            break;
        }
        

    }
    tokens
}