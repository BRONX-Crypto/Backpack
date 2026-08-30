use crate::TokenCreate::Token;
use crate::TokenCreate::mode::*;
use crate::TokenCreate::*;
use bitvec::prelude::*;
pub fn process(tokens: Vec<Token>) {
    let mut IP = 0;
    let mut stack: BitVec<u8, Msb0> = bitvec![u8, Msb0; 0, 0, 0, 1, 1, 1, 0, 1, 1];
    let mut address_ret_stack: BitVec<u8, Msb0> = BitVec::new();
    while IP < tokens.len() {
        match &tokens[IP] {
            Token::nop => {
                IP += 1;
                continue;
            },
            Token::push(s) => {
                        let d = s.clone();
                        for x in d {
                            stack.push(x);
                        } println!("Stack: {:?}", stack);
                        IP += 1;
                        continue;    
                    },
                
            Token::pop => {
                stack.pop();
                IP += 1;
                continue;
            },
            Token::plus => {
                let a: u8 = match stack[stack.len() - 1] {false => 0, _ => 1, };
                let b: u8 = match stack[stack.len() - 2] {false => 0, true => 1, };
                let result: u8 = a + b;
                let bfor = format!("{:b}", result);
                let vec_number_but_char: Vec<char> = bfor.chars().collect();
                let chartonum: BitVec<u8, Msb0> = vec_number_but_char.iter().map(|b| match b { '0' => false, _ => true }).collect();
                for value in chartonum {
                    stack.push(value);
                }
                IP += 1;
                continue;
            },
            Token::minus => {
                let a: u8 = match stack[stack.len() -1] { false => 0, true => 1, _ => 1, };
                let b: u8 = match stack[stack.len() -2] { false => 0, true => 1, _ => 1, };
                let result: u8 = a - b;
                let bfor = format!("{:b}", result);
                let vnbc: Vec<char> = bfor.chars().collect();
                let vtn: BitVec<u8, Msb0> = vnbc.iter().map(|b| match b { '0' => false, _ => true, }).collect();
                for v in vtn {
                    stack.push(v);
                }
                IP += 1;
                continue;
            },
            Token::swap => {
                let len = stack.len();
                stack.swap(len - 1, len - 2);
                IP += 1;
                continue;
            },
            Token::copy => {
                let b = stack[stack.len() - 1];
                stack.push(b);
                IP += 1;
                continue;
            },
            Token::compare => {
                let a: bool = stack[stack.len() - 1];
                let b: bool = stack[stack.len() - 2];
                    if a < b {
                        stack.push(false);
                        stack.push(true);
                    }
                    if a == b {
                        stack.push(false);
                        stack.push(false);
                    }
                    if a > b {
                        stack.push(true);
                        stack.push(false);
                    }
                    IP += 1;
                continue;
            },
            Token::Do(s) => {
                println!("Do on IP: 1");
                        let the_datas = s.clone();
                        IP = the_datas as usize;
                        continue;
                },
            Token::Do_IF(s) => {
                let last = stack[stack.len() -1];
                if last == false {
                    IP += 1;
                }
                if last == true {
                    IP = *s as usize;
                    println!("Do IF Is on the {}", *s)
                }
                continue;
            },
            //obo = open bitwes
            Token::obo(source) => {
                let data = source.clone();
                match data {
                    XOR => {
                        let a = stack[stack.len() - 1];
                        let b = stack[stack.len() - 2];
                        match (a, b) { 
                            (false, true) => {
                                stack.push(true)
                            }, 
                            (true, false) => stack.push(true),
                            _ => stack.push(false),
                        }
                    },
                    AND => {
                        let a = stack[stack.len() - 1];
                        let b = stack[stack.len() - 2];
                        match (a, b) {
                            (true, true) => stack.push(true),
                            _ => stack.push(false),
                        }
                    },
                    OR => {
                        let a = stack[stack.len() - 1];
                        let b = stack[stack.len() - 2];
                        match (a, b) {
                            (false, true) => stack.push(true),
                            (true, false) => stack.push(true),
                            (true, true) => stack.push(true),
                            _ => stack.push(false),
                        }
                    },
                    NOT => {
                        let a = stack[stack.len() - 1];
                        match a {
                            false => stack.push(true),
                            true => stack.push(false),
                            _ => (),
                        }
                    },


                }
            
                
            
                
            IP += 1;
            },
            Token::Done => {
                IP += 1;
                break;
            },
            Token::Duplicate_Select(source) => {
                let takeLast = source.clone();
                let TLN: BitVec<u8, Msb0> = stack[stack.len() - takeLast as usize ..].to_bitvec();
                let mut TLNS = String::new();
                for a in TLN {
                    match a {
                    true => TLNS.push('1'),
                    _ => TLNS.push('0'),
                    };
                }
                let TLNSN = u64::from_str_radix(&TLNS, 2).unwrap();
                let wh: bool = stack[stack.len() - 1 - TLNSN as usize];
                stack.push(wh);
                IP += 1;
                continue;
                },
                Token::swap_Select(source, source0) => {
                    let takeLast= source.clone() as usize;
                    let slice: Vec<char> = stack[stack.len() - takeLast ..].iter().map(|b| match *b { false => '0', _ => '1', }).collect();
                    let mut slicestr = String::new();
                    for unit in slice {
                        slicestr.push(unit);
                    }
                    let slicen = u64::from_str_radix(&slicestr, 2).unwrap();
                    let Index = stack.len() - slicen as usize;
                    let end = Index - source0.clone() as usize;
                    let sn: Vec<char> = stack[end..Index].iter().map(|b| match *b { false => '0', _ => '1', }).collect();
                    let mut sns = String::new();
                    for x in sn {
                        sns.push(x);
                    }
                    let stacklen = stack.len();
                    let snsn = u64::from_str_radix(&sns, 2).unwrap();
                    let _Index = stacklen - snsn as usize;
                    stack.swap(Index, _Index);
                    IP += 1;
                    continue;
                },
                Token::swap_select_to_last(source) => {
                    let takeLast = source.clone() as usize;
                    let slice = stack[stack.len() - takeLast ..].to_bitvec();
                    let slice: Vec<char> = slice.iter().map(|b| match *b { false => '0', _ => '1', }).collect();
                    let mut slice_to_str = String::new();
                    let stacklen = stack.len();
                    for x in slice {
                        slice_to_str.push(x);
                    }
                    let slice_int = u64::from_str_radix(&slice_to_str, 2).unwrap();
                    let Index = stacklen - slice_int as usize;
                    stack.swap(Index, stacklen - 1);
                    IP += 1;
                    continue;
                },
                Token::call(source) => {
                    let takeLast = source.clone() as usize;
                    let slice = stack[stack.len() - takeLast ..].to_bitvec();
                    let char_slice: Vec<char> = slice.iter().map(|b| match *b { false => '0', _ => '1', }).collect();
                    let mut string_slice = String::new();
                    for ch in char_slice {
                        string_slice.push(ch);
                    }
                    let number = u64::from_str_radix(&string_slice, 2).unwrap();
                    let returnadr = format!("{:b}", IP + 1);
                    for cha in returnadr.chars() {
                        match cha {
                            '0' => address_ret_stack.push(false),
                            _ => address_ret_stack.push(true),
                        };
                    }
                    IP = number as usize;

                },
                Token::ret => {
                    let read_Vec: Vec<char> = address_ret_stack.iter().map(|b| match *b { false => '0', _ => '1', }).collect();
                    let mut read_String = String::new();
                    for ch in read_Vec {
                        read_String.push(ch);
                    }
                    let numb = u64::from_str_radix(&read_String, 2).unwrap();
                    address_ret_stack.clear();
                    IP = numb as usize;
                    continue;
                },
            _ => (),
        }
        
                
                
            

    
        
    }
    println!("Stack: {:?}", stack);
    }