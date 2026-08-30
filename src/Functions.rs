use bitvec::prelude::*;
pub fn read_to_u64(vector: &BitVec<u8, Msb0>, mut i: usize) -> (u64, usize) {
    let mut start = i;
    i += 4;
    let vect = &vector[start..=i];
    let e = to_u64(&vect);
    start = i;
    for _ in 0..e {
        i += 1;
    }
    let vect0 = &vector[start+1..=i];
    let _e = to_u64(&vect0);
    start = i;
    for _ in 0.._e {
        i += 1;
    }
    let vect01 = &vector[start+1..=i];
    (to_u64(&vect01), i + 1)
}
pub fn to_u64(slice: &BitSlice<u8, Msb0>) -> u64 {
    let mut value = 0;
    for bit in slice {
        value = value * 2 + (*bit as u64);
    }
    value
}