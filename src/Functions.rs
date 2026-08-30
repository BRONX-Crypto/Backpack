use bitvec::prelude::*;
pub fn read_to_u64(vector: &BitVec<u8, Msb0>, mut i: usize) -> (u64, usize) {
    let mut start = i;
    i += 4;
    let vect: Vec<char> = vector[start..=i].iter().map(|b| match *b { false => '0', _ => '1', }).collect();
    let mut _str = String::new();
    for x in vect {
        _str.push(x);
    }
    let e = u64::from_str_radix(&_str, 2).unwrap();
    start = i;
    for _ in 0..e {
        i += 1;
    }
    let vect0: Vec<char> = vector[start+1..=i].iter().map(|b| match *b { false => '0', _ => '1', }).collect();
    let mut __str = String::new();
    for c in vect0 {
        __str.push(c);
    }
    let _e = u64::from_str_radix(&__str, 2).unwrap();
    start = i;
    for _ in 0.._e {
        i += 1;
    }
    let vect01: Vec<char> = vector[start+1..=i].iter().map(|b| match *b { false => '0', _ => '1', }).collect();
    let mut ___str = String::new();
    for new in vect01 {
        ___str.push(new);
    }
    (u64::from_str_radix(&__str, 2).unwrap(), i + 1)
}