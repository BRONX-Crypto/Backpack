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
use crate::TokenCreate::option3::*;
use crate::TokenCreate::option3;
use crate::TokenCreate::option0::*;
use crate::TokenCreate::option::*;
use std::collections::HashMap;
use crate::TokenCreate::Token::HeapConfigurationCollection;
use crate::TokenCreate::hci_Option::*;
use crate::TokenCreate::soh_option::*;
pub fn process(tokens: Vec<Token>) {
    let mut IP = 0;
    let mut stack: BitVec<u8, Msb0> = BitVec::new();
    let mut address_ret_stack: BitVec<u8, Msb0> = BitVec::new();
    let mut cut_stack: BitVec<u8, Msb0> = BitVec::new();
    let mut second_count_cut_stack: BitVec<u8, Msb0> = BitVec::new();
    let mut source_select_bool: bool = false;
    let mut save_select_bool: bool = false;
    let mut BlockSizeStack: BitVec<u8, Msb0> = BitVec::new();
    let mut SecondSizeStack: BitVec<u8, Msb0> = BitVec::new();
    let mut bob = true;
    let mut heap: HashMap<u64, bool> = HashMap::new();
    let mut save_adr: u64 = 0;
    let mut blocksizeofheap: u64 = 0;
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
                match bob {
                    false => {
                let a: u8 = match stack[stack.len() - 1] {false => 0, _ => 1, };
                let b: u8 = match stack[stack.len() - 2] {false => 0, true => 1, };
                let result: u8 = a + b;
                let bfor = format!("{:b}", result);
                let vec_number_but_char: Vec<char> = bfor.chars().collect();
                let chartonum: BitVec<u8, Msb0> = vec_number_but_char.iter().map(|b| match b { '0' => false, _ => true }).collect();
                if save_select_bool == false {
                    stack.pop(); stack.pop();
                }
                if save_select_bool == true {
                    ()
                }
                for value in chartonum {
                    stack.push(value);
                }
                },
                _ => {
                    let r = to_u64(&cut_stack);
                    let r2 = to_u64(&second_count_cut_stack);
                    let last = stack.len();
                    let range = &stack[last - r as usize..last];
                    let rintonum = to_u64(&range);
                    let l = last - r as usize- r2 as usize;
                    let new= &stack[l..stack.len() - r as usize];
                    let tu64 = to_u64(&new); //[1, 0, 1, 0]
                    let sum = rintonum + tu64;
                    let sumf = format!("{:b}", sum);
                    if save_select_bool == false {
                        stack.truncate(stack.len() - r as usize -r2 as usize);
                    }
                    else {
                        ()
                    }
                    for x in sumf.chars() {
                        match x {
                            '0' => stack.push(false),
                            _ => stack.push(true),
                        }
                    }
                },
            }
                IP += 1;
                continue;
            },
            Token::minus => {
                match bob {
                    false => {
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
            },
            _ => {
let r = to_u64(&cut_stack);                                               let r2 = to_u64(&second_count_cut_stack);                                 let last = stack.len();                                                   let range = &stack[last - r as usize..last];                              let rintonum = to_u64(&range);                                            let l = last - r as usize- r2 as usize;                                   let new= &stack[l..stack.len() - r as usize];                             let tu64 = to_u64(&new);                            let sum = rintonum - tu64;                                                let sumf = format!("{:b}", sum);
                    if save_select_bool == false {                                                stack.truncate(stack.len() - r as usize -r2 as usize);                                                     }                                    else {                                   ()                               }                                    for x in sumf.chars() {                                                       match x {                                '0' => stack.push(false),                                                 _ => stack.push(true),                                                }                                }                







},
        }
                IP += 1;
                continue;
    },
            Token::swap => {
                if bob == false {
                let len = stack.len();
                stack.swap(len - 1, len - 2);
                }
                else {
                    let f = to_u64(&cut_stack) as usize;
                    let sec = to_u64(&second_count_cut_stack) as usize;
                    let f_range = stack[stack.len() - 1 - f..stack.len()].to_bitvec().clone();
                    let sec_range = stack[stack.len() - 1 - f - sec..f].to_bitvec().clone();
                    for (i, x) in f_range.clone().into_iter().enumerate() {
                        for (j, y) in sec_range.clone().into_iter().enumerate() {
                            stack.swap(i, j);
                        }
                    }
                }
                IP += 1;
                continue;
            },
            Token::copy => {
                if bob == false {
                let b = stack[stack.len() - 1];
                stack.push(b);
                }
                else {
                    let f = to_u64(&cut_stack);
                    let range = stack[stack.len() - 1 - f as usize..stack.len()].to_bitvec().clone();
                    for x in range.clone() {
                        stack.push(x);
                    }
                }
                IP += 1;
                continue;
            }
            Token::compare => {
                if !bob {
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
                }
                else {
                    let f = to_u64(&cut_stack);
                    let sec = to_u64(&second_count_cut_stack);
                    let FirstR = &stack[stack.len() - 1 - f as usize..stack.len()];
                    let SecRange = &stack[stack.len() - 1 - f as usize - sec as usize..f as usize];
                    let firstrn = to_u64(&FirstR);
                    let secrangen = to_u64(&SecRange);
                    if firstrn == secrangen {
                        stack.extend([false, false]);
                    }
                    if firstrn < secrangen {
                        stack.extend([false, true]);
                    }
                    else {
                        stack.extend([true, false]);
                    }
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
                        Token::Do_IF(option::FromStack) => {
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
                        clear(onHeap) => {
                            heap.clear(); },
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
                    cut_stack.clear();
                    for x in &*v {
                        cut_stack.push(*x);
                    }
                    IP += 1;
                    continue
                    },
                Token::set_2nd_cs(v) => {
                    second_count_cut_stack.clear();
                        second_count_cut_stack.clear();
                        for x in &*v {
                            second_count_cut_stack.push(*x);
                    }
                    IP += 1;
                    continue;
                },
                Token::BlockOrBit(_) => {
                    match tokens[IP] {
                        BlockOrBit(Bit) => bob = false,
                        _ => bob = true,
                    }
                    IP += 1;
                    continue;
                },
                HeapConfigurationCollection(_) => {
                    match &tokens[IP] {
                        HeapConfigurationCollection(set_adr(n)) => {
                            save_adr = *n;
                        },
                        HeapConfigurationCollection(configBlockSizeHeapBlock(n)) => {
                        blocksizeofheap = *n;
                        },
                        HeapConfigurationCollection(MakeContent(content)) => {
                            let mut counter: u64 = 0;
                               for x in content {
                                heap.insert(save_adr + counter, *x);
                                counter += 1;
                            }
                        },
                        HeapConfigurationCollection(DeleteContent) => {
                            let mut count: u64 = 0;
                            let mut sum = 0;
                            while count < blocksizeofheap {
                                sum = save_adr + count;
                                heap.remove(&sum);
                                count += 1;
                            }
                        }
                        _ => (),
                    }
                    IP += 1;
                    continue;
                },
                clone(_) => {
                    match tokens[IP] {
                        clone(Stack) => {
                            if !bob {
                        let r: bool = stack[stack.len() - 1];
                        heap.insert(save_adr, r);
                        }
                        else {
                            let tu64 = to_u64(&cut_stack) as usize;
                            let range = &stack[stack.len() - 1 - tu64..stack.len()];
                            let mut counter = 0;
                            while counter < blocksizeofheap {
                                heap.insert(save_adr + counter as u64, range[counter as usize]);
                            }
                        }
                    },
                        _ => {
                                if !bob {
                                    let v: bool = heap.get(&save_adr).is_some();
                                    stack.push(v);
                                }
                                else {
                                    let mut counter = 0;
                                    while counter < blocksizeofheap as usize {
                                        let v: bool = heap.get(&(save_adr + counter as u64)).is_some();
                                        stack.push(v);
                                    }
                                }
                        },
                    }
                    IP += 1;
                    continue;
                }


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
println!("Heap: {:?}", heap);
}
