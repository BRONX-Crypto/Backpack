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
    Do(option),
    Do_IF(option),
    obo(mode),
    Done,
    Duplicate_Select(option),
    swap_Select(option2),
    swap_select_to_last(option),
    call(option),
    ret,
    clear(clearmodes),
    Select_Save(ssm),
    set_stack_cut(BitVec<u8 Msb0>),
    set_2nd_cs(BitVec<u8, Msb0>),
    //lss clear
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ssm {
    ssm_Save,
    ssm_non_save,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum option {
    FromIn(u64),
    FromStack,
} use option::*;
//vectok = vector to token
pub enum option2 {
    FromIn(u64, u64),
    From_Stack,
}
pub fn vectok(vector: BitVec<u8, Msb0>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut bss: BitVec<u8, Msb0> = BitVec::new();
    let mut StackorL = false;
    //True Mean Read From Stack, And False Mean Read From InLine
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
        if vector[i] == false && vector[i+1] == false && vector[i+2] == true && vector [i+3] == true && vector[i+4] == false {
            tokens.push(Token::copy);
            i += 5;
            continue;
        }
        if vector[i] == false && vector[i+1] == false && vector[i+2] == true && vector[i+3] == true && vector[i+4] == true{
            tokens.push(Token::compare);
            i += 5;
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == false && vector[i+3] == false && vector[i+4] == false {
            i += 5;
            if StackOrL == false {

            
            let (data, _i) = read_to_u64(&vector, &bss, i);
            tokens.push(Token::Do(option::FromIn(data)));
            i = _i;
            }
            if StackOrL == true {
                tokens.push(Token::Do(option::FromStack));
                i += 5;
            }
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == false && vector[i+3] == false && vector[i+4] == true {
            i += 5;
            if StackOrL == false {
            let (ipnumber, _i) = read_to_u64(&vector, &bss, i);
            tokens.push(Token::Do_IF(option::FromIn(ipnumber)));
            i = _i;
            }
            else {
                tokens.push(Token::Do_IF(option::FromStack))
            }
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == false && vector[i+3] == true && vector[i+4] == false {
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
        if !vector[i] && vector[i+1] && !vector[i+2] && vector[i+3] && vector[i+4] {
            tokens.push(Token::Done);
            i += 5;
            continue;
        }
        if !vector[i] && vector[i+1] && vector[i+2] && !vector[i+3] && !vector[i+4] {
            i += 5;
            if StackOrL == false {
            let (data, _i) = read_to_u64(&vector, &bss, i);
            i = _i;
            tokens.push(Token::Duplicate_Select(option::FromIn(data)));
            }
            else {
                tokens.push(Token::Duplicate_Select(option::FromStack));
            }
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == true && vector[i+3] == false && vector[i+4] == true {
            i += 5;
            if StackOrL == false {
            let (fd, i0) = read_to_u64(&vector, &bss, i);
            let (sd, i1) = read_to_u64(&vector, &bss, i0);
            i = i1;
            tokens.push(Token::swap_Select(option2::FromIn(fd, sd)));
            }
            else {
                tokens.push(Token::swap_Select(option2::From_Stack));
            }
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == true && vector[i+3] == true && vector[i+4] == false {
            i += 5;
            if StackOrL == false {
            let (data, _i) = read_to_u64(&vector, &bss, i);
            tokens.push(Token::swap_select_to_last(option::FromIn(data)));
            i = _i;
            }
            else {
                tokens.push(Token::swap_select_to_last(option::FromStack))
            }
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == true && vector[i+3] == true && vector[i+4] == true {
            if StackOrL == false {
            i += 5;
            let (data, _i) = read_to_u64(&vector, &bss, i);
            tokens.push(Token::call(option::FromIn(data)));
            i = _i;
            }
            else {
                tokens.push(Token::call(option::FromStack))
            }
            continue;
        }
        if vector[i] == true && vector[i+1] == false && vector[i+2] == false && vector[i+3] == false && vector[i+4] == false{
            i += 5;
            tokens.push(Token::ret);
            continue;
        }
        if vector[i] == true && vector[i+1] == false && vector[i+2] == false && vector[i+3] == true && vector[i+4] == true {
            i += 5;
            if vector[i] == false {
                tokens.push(Token::clear(onStack));
            }
            if vector[i] == true {
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
        if vector[i] == true && vector[i+1] == false && vector[i+2] == true && vector[i+3] == false && vector[i+4] == true {
            i += 1;
            if vector[i] == false {
                tokens.push(Token::Select_Save(ssm::ssm_Save));
            }
            else { 
                tokens.push(Token::Select_Save(ssm::ssm_non_save));
            }
            i += 1;
            continue;
        }
        if vector[i] == true && vector[i+1] == false && vector[i+2] == true && vector[i+3] == true && vector[i+4] == false {
            let (data, _i) = read_to_vec(&vector, &bss, i);
            tokens.push(Token::set_stack_cut(data));
            i = _i;
            continue;
        }
        if vector[i] == true && vector[i+1] == true && vector[i+2] == false && vector[i+3] == false && vector[i+4] == false {
            StackOrL = false;
            i += 5;
            continue;
        }
        if vector[i] == true && vector[i+1] == false && vector[i+2] == false && vector[i+2] == true && vector[i+4] == false {
                let (data, _i) = read_to_vec(&vector, &bss, i);
            tokens.push(Token::set_2nd_cs(data));
            i += 5;
            continue;
    }
    if vector[i] == true && vector[i+1] == true && vector[i+2] == false && vector[i+3] == false && vector[i+4] == true {
        bss.clear();
        i += 5;
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
