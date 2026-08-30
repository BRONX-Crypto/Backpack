use bitvec::prelude::*;
use crate::Functions::*;
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
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum mode {
    XOR,
    AND,
    OR,
    NOT,
} use mode::*;
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
            let chars: Vec<char> = vector[start..=i].iter().map(|b| match *b { false => '0', true => '1', _ => '!', }).collect();
        // cts = chars to string
        let cts: String = chars.into_iter().collect();
        let number = u64::from_str_radix(&cts, 2).unwrap();
        start = i;
        for _ in 0..number as usize {
            i += 1;
        }
        let len: Vec<char> = vector[start+1..=i].iter().map(|b| match *b { false => '0', _ => '1',}).collect();
        let cts0: String = len.into_iter().collect();
        let number0 = u64::from_str_radix(&cts0, 2).unwrap();
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
            let mut dataa: Vec<char> = vector[start..=i].iter().map(|b| match *b { false => '0', true => '1', _ => '1', }).collect();
            let mut dataas = String::new();
            for o in dataa {
                dataas.push(o);
            }
            let nm = u64::from_str_radix(&dataas, 2).unwrap();
            start = i;
            for _ in 0..nm as usize {
                i += 1;
            }
            let datat: Vec<char> = vector[start+1..=i].iter().map(|b| match *b { false => '0', true => '1', _ => '1'}).collect();
            let mut dattas = String::new();
            for k in datat {
                dattas.push(k);
            }
            //ndattas = number dattas
            let ndattas = u64::from_str_radix(&dattas, 2).unwrap();
            start = i;
            for _ in 0..ndattas as usize {
                i += 1;
            }
            let the_ip_datach: Vec<char> = vector[start+1..=i].iter().map(|b| match *b { false => '0', true => '1', _ => '1'}).collect();
            let mut the_data_ip_string = String::new();
            for y in the_ip_datach {
                the_data_ip_string.push(y);
            }
            let nmb = u64::from_str_radix(&the_data_ip_string, 2).unwrap();
            tokens.push(Token::Do(nmb));
            i += 1;
            continue;
        }
        if vector[i] == false && vector[i+1] == true && vector[i+2] == false && vector[i+3] == true && vector[i+4] == false {
            i += 5;
            let mut start = i;
            i += 4;
            let mut lenlens = String::new();
            let lenlen: Vec<char> = vector[start..=i].iter().map(|b| match *b { false => '0', true => '1', _ => '1', }).collect();
            for vb in lenlen {
                lenlens.push(vb);
            }
            let lelensn = u64::from_str_radix(&lenlens, 2).unwrap();
            start = i;
            for _ in 0..lelensn {
                i += 1;
            }
            let mut lenvcs = String::new();
            let lenvc: Vec<char> = vector[start+1..=i].iter().map(|b| match *b { false => '0', true => '1', _ => '1', }).collect();
            for fx in lenvc {
                lenvcs.push(fx);
            }
            let numb = u64::from_str_radix(&lenvcs, 2).unwrap();
            start = i;
            for _ in 0..numb {
                i += 1;
            }
            let mut tstringip = String::new();
            let the_ip_data: Vec<char> = vector[start+1..=i].iter().map(|b| match *b { false => '0', true => '1', _ => '1'}).collect();
            for gh in the_ip_data {
                tstringip.push(gh);
            }
            let ipnumber = u64::from_str_radix(&tstringip, 2).unwrap();
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
            let mut st = String::new();
            let sti: Vec<char> = vector[start..=i].iter().map(|b| match *b { false => '0', _ => '1', }).collect();
            for cz in sti {
                st.push(cz);
            }
            let stn = u64::from_str_radix(&st, 2).unwrap();
            start = i;
            for _ in 0..stn {
                i += 1;
            }
            let mut lens = String::new();
            let len: Vec<char> = vector[start+1..=i].iter().map(|b| match *b { false => '0', true => '1', _ => '1' }).collect();
            for er in len {
                lens.push(er);
            }
            let lensn = u64::from_str_radix(&lens, 2).unwrap();
            start = i;
            for _ in 0..lensn {
                i += 1;
            }
            let vecch: Vec<char> = vector[start+1..=i].iter().map(|b| match *b { false => '0', _ => '1',  }).collect();
            let mut vecstring = String::new();
            for rg in vecch {
                vecstring.push(rg);
            }
            let data = u64::from_str_radix(&vecstring, 2).unwrap();
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
        
        

    }
    tokens
}