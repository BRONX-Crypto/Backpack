#[allow(warnings)]
use crate::TokenCreate::Token;
use crate::TokenCreate::mode::*;
use bitvec::prelude::*;
use crate::Functions::*;
use crate::TokenCreate::Token::*;
use crate::TokenCreate::clearmodes::*;
use crate::TokenCreate::ssm::*;
use crate::TokenCreate::option::*;
use crate::TokenCreate::option2::*;
use crate::TokenCreate::option;
use crate::TokenCreate::option2;
pub fn process(tokens: Vec<Token>) {
    let mut IP = 0;
    let mut stack: BitVec<u8, Msb0> = BitVec::new();
    let mut address_ret_stack: BitVec<u8, Msb0> = BitVec::new();
    let mut cut_stack: BitVec<u8, Msb0> = BitVec::new();
    let mut second_count_cut_stack: BitVec<u8, Msb0> = BitVec::new();
    let mut source_select_bool: bool = false;
    let mut save_select_bool: bool = false;
    while IP < tokens.len() {
        match &tokens[IP] {
            Token::nop => { IP += 1;
                continue;
            },
            Token::push(s) => {
                        let d = s.clone();
                        for x in d {
                            stack.push(x);
                        }
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
                if save_select_bool == false {
                    stack.pop(); stack.pop();
                }
                if save_select_bool == false {
                    ()
                }
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
                if save_select_bool == false {
                    stack.pop(); stack.pop();
                }
                else {
                    ()
                }
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
                if save_select_bool == false {
                    stack.pop(); stack.pop();
                } else { ( ) }
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
            Token::Do(_) => {
                match tokens[IP] {
                    Token::Do(option::FromIn(s)) => {
                println!("Do on IP: {}", s.clone());
                        let the_datas = s.clone();
                        IP = the_datas as usize;
                    },
                    _ => {
                        let intonum = to_u64(&cut_stack);
                        let range = &stack[stack.len() - intonum as usize ..];
                        let rangeInt = to_u64(&range);
                        if save_select_bool == false {
                            stack.truncate(stack.len() - intonum as usize);
                            IP = rangeInt as usize;
                        }
                        else {
                            IP = rangeInt as usize;
                        }




                    },
                }
                continue;
                },
            Token::Do_IF(_) => {
                let last = stack[stack.len() -1];
                if last == false {
                    IP += 1;
                }
                if last == true {
                    match tokens[IP] {
                        Token::Do_IF(FromStack) => {
                            let cloneToNumber = to_u64(&cut_stack);
                            let range = &stack[stack.len() - cloneToNumber as usize ..];
                            let rangeInt = to_u64(&range);
                            if save_select_bool == false {
                                stack.truncate(stack.len() - cloneToNumber as usize);
                                IP = rangeInt as usize;
                            }
                            else {
                                IP = rangeInt as usize;
                            }
                        },
                        Do_IF(option::FromIn(n)) => {
                            IP = n.clone() as usize;
                        },
                        _ => (),
                    }
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
                                if save_select_bool == true {stack.push(true);}
                                if save_select_bool == false { stack.pop(); stack.pop(); stack.push(true); }
                            }, 
                            (true, false) => {if save_select_bool == false {stack.pop(); stack.pop(); stack.push(true); } else { stack.push(false)}},
                            _ => {
                                match save_select_bool {
                                    false => {
                                        stack.pop(); stack.pop(); stack.push(false);
                                    },
                                    _ => stack.push(false),
                                }
                            },
                        }
                    },
                    AND => {
                        let a = stack[stack.len() - 1];
                        let b = stack[stack.len() - 2];
                        match (a, b) {
                            (true, true) => { match save_select_bool { false => { stack.pop(); stack.pop(); stack.push(true); }, _ => { stack.push(true)}, } }

                            _ => { match save_select_bool { false => { stack.pop(); stack.pop(); stack.push(false) }, _ => stack.push(false), } }
                        }
                    },
                    OR => {
                        let a = stack[stack.len() - 1];
                        let b = stack[stack.len() - 2];
                        match (a, b) {
                            (false, true) => {match save_select_bool { false => { stack.pop(); stack.pop(); stack.push(true); }, _ => stack.push(true), } },
                            (true, false) => {match save_select_bool { false => { stack.pop(); stack.pop(); stack.push(true); }, _ => stack.push(true), } },
                            (true, true) => {match save_select_bool { false => { stack.pop(); stack.pop(); stack.push(true); }, _ => { stack.push(true); } } },
                            _ => { match save_select_bool { false => { stack.pop(); stack.pop(); stack.push(false); }, _ => stack.push(false), } },
                        }
                    },
                    NOT => {
                        let a = stack[stack.len() - 1];
                        match a {
                            false => { match save_select_bool { false => { stack.pop(); stack.push(true); }, _ => stack.push(true), } },
                            true => { match save_select_bool { false => { stack.pop(); stack.push(false); }, _ => stack.push(false), } },
                            _ => (),
                        }
                    },
                    _ => ()
                }
                IP += 1;
                continue;
            },
            Token::Done => {
                break;
            },
            Token::Duplicate_Select(_) => {
                match tokens[IP] {
                    Duplicate_Select(option::FromStack) => {
                let takeLast = to_u64(&cut_stack);
                let wh = &stack[stack.len() - takeLast as usize ..];
                let into_u64 = to_u64(&wh);
                let Index = stack.len() - into_u64 as usize;
                if save_select_bool == false {
                    stack.truncate(stack.len() - takeLast as usize);
                    let cop = stack[Index];
                    stack.push(cop);
                }
                else {
                    let cop = stack[Index];
                    stack.push(cop);
                }
                },
                Duplicate_Select(option::FromIn(s)) => {
                    let Index = stack[stack.len() - s.clone() as usize];
                    stack.push(Index);
                },
                _ => (),
            }
                IP += 1;
                continue;
                },
                Token::swap_Select(_) => {
                    match tokens[IP] {
                        Token::swap_Select(option2::From_Stack) => {
                    let takeLast = to_u64(&cut_stack);
                    let source0 = to_u64(&second_count_cut_stack);
                    let slice = &stack[stack.len() - takeLast as usize ..];
                    let slicen = to_u64(&slice);
                    if save_select_bool == false {
                        stack.truncate(stack.len() - slicen as usize);
                    } else { () }
                    let Index = stack.len() - slicen as usize;
                    let end = Index - source0.clone() as usize;
                    let sn = &stack[end..Index];
                    let stacklen = stack.len();
                    let snsn = to_u64(&sn);
                    let _Index = stacklen - snsn as usize;
                    stack.swap(Index, _Index);
                    },
                    Token::swap_Select(option2::FromIn(s, s2)) => {
                        let o = stack.len() - s.clone() as usize;
                        let t = stack.len() - s2.clone() as usize;
                        stack.swap(o, t);
                    },
                    _ => (),


                    }
                    IP += 1;
                    continue;
                },
                Token::swap_select_to_last(_) => {
                    match tokens[IP] {
                        swap_select_to_last(option::FromStack) => {
                    let takeLast = to_u64(&cut_stack);
                    let slice = stack[stack.len() - takeLast as usize ..].to_bitvec();
                    let stacklen = stack.len();
                    let slice_int = to_u64(&slice);
                    if save_select_bool == false {
                        stack.truncate(stacklen.clone() - slice_int as usize);
                    }
                    else { () }
                    let Index = stacklen - slice_int as usize;
                    stack.swap(Index, stacklen - 1);
                    },
                    swap_select_to_last(option::FromIn(s)) => {
                        let stack_l = stack.len();
                        stack.swap((stack_l.clone() - s.clone() as usize), stack_l - 1);
                    },
                    _ => (),
                }
                    IP += 1;
                    continue;
                },
                Token::call(_) => {
                    match tokens[IP] {
                        call(option::FromStack) => {
                    let takeLast = to_u64(&cut_stack);
                    let slice = stack[stack.len() - takeLast as usize ..].to_bitvec();
                    if save_select_bool == false {
                    stack.truncate(stack.len() - takeLast as usize);
                    }
                    else { () }
                    let number = to_u64(&slice);
                    let ipn = stack.len() - number as usize;
                    let returnadr = format!("{:b}", IP + 1);
                    for cha in returnadr.chars() {
                        match cha {
                            '0' => address_ret_stack.push(false),
                            _ => address_ret_stack.push(true),         };
                    }
                    IP = ipn as usize;
                },
                call(option::FromIn(s)) => {
                    IP = s.clone() as usize;
                    let f = format!("{:b}", IP + 1);
                    for item in f.chars() {
                        match item {
                            '0' => address_ret_stack.push(false),
                            '1' => address_ret_stack.push(true),
                            _ => (),
                        }
                    }
                },
                _ => (),
                }
                continue;
                },
                Token::ret => {
                    let numb = to_u64(&address_ret_stack);
                    address_ret_stack.clear();
                    IP = numb as usize;
                    continue;
                },
                Token::clear(_) => {
                    match tokens[IP] {
                        clear(onStack) => {
                            stack = BitVec::new();
                        },
              |          clear(onHeap) => {
                            println!("Clear Heap not added Becuse on this version heap it's not real");
                        },
                        _ => todo!(),
                    }
                    IP += 1;
                    continue;
                },
                Token::Select_Save(_) => {
                    match tokens[IP] {
                        Select_Save(ssm_Save) => {
                            save_select_bool = false;
                        },
                        Select_Save(ssm_non_save) => {
                            save_select_bool = true;
                        },
                        _ => (),
                    }
                    IP += 1;
                    continue;
                },
                Token::set_stack_cut(v) => {
                    for x in &*v {
                        second_count_cut_stack.push(*x);
                    }
                    IP += 1;
                    continue
                    },
                Token::set_2nd_cs(v) => {
                        second_count_cut_stack.clear();
                        for x in &*v {
                            second_count_cut_stack.push(*x);
                    }
                    IP += 1;
                    continue;
                },


            _ => (),
        }

        
                
                
            

    
    }
    let vec_stack: Vec<char> = stack.iter().map(|b| match *b { false => '0', _ => '1', }).collect();
    println!("Stack: ");
    print!("(");
    for x in vec_stack {
        print!("{}", x);
    }
println!(")");

}
