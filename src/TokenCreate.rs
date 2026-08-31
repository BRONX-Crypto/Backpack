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
    clear()
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum mode {
    XOR,
    AND,
    OR,
    NOT,
} use mode::*;
#[derive(Debug, Clone, )]
pub enum clearmodes {
    onStack,
    onHeap,
}
#[derive(Eq, Debug, PartialEq, Clone)]
pub enum popmode {
    FromStack(u64),
    FromInLine(u64),
} use popmode::*;
//vectok = vector to token
pub fn vectok(vector: BitVec<u8, Msb0>) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < vector.len() {
        if !vector[i] && !vector[i + 1] && !vector[i + 2] && !vector[i+3] && !vector[i+4] {
            tokens.push(Token::nop);
            i += 5;
            continue;
        }
        if !vector[i] && !vector[i+1] && !vector[i+2] && !vector[i+3] && vector[i+4] {
            i += 5;
            let mut start = i;
            i += 4;
            let slice0 = &vector[start..=i];
        let number = to_u64(&slice0);
        start = i;
        for _ in 0..number as usize {
            i += 1;
        }
        let len0 = &vector[start+1..=i];
        let number0 = to_u64(&len0);
        start = i;
        for _ in 0..number0 as usize {
            i += 1;
        }
        let data: BitVec<u8, Msb0> = vector[start+1..=i].to_bitvec();
        tokens.push(Token::push(data));
        i += 1;
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
            let mut start = i;
            i += 4;
            let mut dataa = &vector[start..=i];
            let nm = to_u64(&dataa);
            start = i;
            for _ in 0..nm as usize {
                i += 1;
            }
            let datat = &vector[start+1..=i];
            let ndattas = to_u64(&datat);
            //ndattas = number dattas
            start = i;
            for _ in 0..ndattas as usize {
                i += 1;
            }
            let the_ip_datach = &vector[start+1..=i];
            let nmb = to_u64(&the_ip_datach);
            tokens.push(Token::Do(nmb));
            i += 1;
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == false && vector[i+3] == true && vector[i+4] == false {
            i += 5;
            let mut start = i;
            i += 4;
            let lenlen = &vector[start..=i];
            let lelensn = to_u64(&lenlen);
            start = i;
            for _ in 0..lelensn {
                i += 1;
            }
            let lenvc = &vector[start+1..=i];
            let numb = to_u64(&lenvc);
            start = i;
            for _ in 0..numb {
                i += 1;
            }
            let ipnumner = &vector[start+1..=i];
            let ipnumber = to_u64(&ipnumner);
            tokens.push(Token::Do_IF(ipnumber));
            i += 1;
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
            let mut start = i;
            i += 4;
            let sti = &vector[start..=i];
            let stn = to_u64(&sti);
            start = i;
            for _ in 0..stn {
                i += 1;
            }
            let len = &vector[start+1..=i];
            let lensn = to_u64(&len);
            start = i;
            for _ in 0..lensn {
                i += 1;
            }
            let vecch = &vector[start+1..=i];
            let data = to_u64(&vecch);
            tokens.push(Token::Duplicate_Select(data));
            i += 1;
            continue
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == true && vector[i+3] == true && vector[i+4] == false {
            i += 5;
            let (fd, i0) = read_to_u64(&vector, i);
            let (sd, i1) = read_to_u64(&vector, i0);
            i = i1;
            tokens.push(Token::swap_Select(fd, sd));
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == true && vector[i+3] == true && vector[i+4] == true {
            i += 5;
            let (data, _i) = read_to_u64(&vector, i);
            tokens.push(Token::swap_select_to_last(data));
            i = _i;
            continue;
        }
        if vector[i] == true && vector[i+1] == false && vector[i+2] == false && vector[i+3] == false && vector[i+4] == false {
            i += 5;
            let (data, _i) = read_to_u64(&vector, i);
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
                let (takeLast, _i) = read_to_u64(&vector, i);
                tokens.push(Token::pop_select_fs(FromInLine(takeLast)));
                i = _i;
            }
            else if vector[i] == true {
                i += 1;
                let (takeLast, _i) = read_to_u64(&vector, i);
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
        else {
            print!("{}", make_colors_rgb("Lexer:", (255, 0, 0), None));
            print!("{}", make_colors_rgb(" This binary data not matches with any opcode", (255, 0, 0), None));
            break;
        }
        

    }
    tokens
}