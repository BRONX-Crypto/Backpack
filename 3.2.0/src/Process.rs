use crate::TokenCreate::Token;
use crate::TokenCreate::mode::*;
use crate::TokenCreate::*;
use bitvec::prelude::*;
pub fn process(tokens: Vec<Token>) {
    let mut IP = 0;
    let mut stack= bitvec![u8, Msb0; 1, 0, 1];

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


                _ => (),
                }
            
                
            
                
            IP += 1;
            },
            Token::Done => {
                IP += 1;
                break;
            },
        }
                
                
            

    }
        
    
    println!("Stack: {:?}", stack);
}