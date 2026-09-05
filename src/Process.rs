#[allow(warnings)]
use crate::TokenCreate::Token;
use crate::TokenCreate::mode::*;
use crate::TokenCreate::*;
use bitvec::prelude::*;
use crate::Functions::*;
use crate::TokenCreate::popmode::*;
use crate::TokenCreate::Token::*;
use crate::TokenCreate::clearmodes::*;
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
                    let slice = &stack[stack.len() - takeLast ..];
                    let slicen = to_u64(&slice);
                    let Index = stack.len() - slicen as usize;
                    let end = Index - source0.clone() as usize;
                    let sn = &stack[end..Index];
                    let stacklen = stack.len();
                    let snsn = to_u64(&sn);
                    let _Index = stacklen - snsn as usize;
                    stack.swap(Index, _Index);
                    IP += 1;
                    continue;
                },
                Token::swap_select_to_last(source) => {
                    let takeLast = source.clone() as usize;
                    let slice = stack[stack.len() - takeLast ..].to_bitvec();
                    let stacklen = stack.len();
                    let slice_int = to_u64(&slice);
                    let Index = stacklen - slice_int as usize;
                    stack.swap(Index, stacklen - 1);
                    IP += 1;
                    continue;
                },
                Token::call(source) => {
                    let takeLast = source.clone() as usize;
                    let slice = stack[stack.len() - takeLast ..].to_bitvec();
                    let number = to_u64(&slice);
                    let ipn = stack.len() - number as usize;
                    let returnadr = format!("{:b}", IP + 1);
                    for cha in returnadr.chars() {
                        match cha {
                            '0' => address_ret_stack.push(false),
                            _ => address_ret_stack.push(true),
                        };
                    }
                    IP = ipn as usize;

                },
                Token::ret => {
                    let numb = to_u64(&address_ret_stack);
                    address_ret_stack.clear();
                    IP = numb as usize;
                    continue;
                },
                Token::pop_select_fs(FromInLine(value)) => {
                            for _ in 0..value.clone() as usize {
                                stack.pop();
                            }
                
                    IP += 1;
                    continue;
                },
                Token::pop_select_fs(FromStack(value)) => {
                    let Index = stack.len() - value.clone() as usize;
                    let sl = &stack[Index..=stack.len()];
                    let sln = to_u64(&sl);
                    let staclen = &stack.len();
                    let _Point = staclen.clone() - sln as usize;
                    let rng = staclen.clone() - _Point;
                    let real_rng = stack[rng..=staclen.clone()].to_bitvec().clone();
                    for _ in real_rng {
                        &stack.pop();
                    }
                    IP += 1;
                    continue;
                }
                Token::clear(_) => {
                    match tokens[IP] {
                        clear(onStack) => {
                            stack = BitVec::new();
                        },
                        clear(onHeap) => {
                            println!("Clear Heap not added Becuse on this version heap it's not real");
                        },
                        _ => todo!(),
                    }
                    IP += 1;
                    continue;
                }
            _ => (),
        }
        
                
                
            

    
        
    }
    println!("Stack: {:?}", stack);
    }