#[allow(warnings)]
use bitvec::prelude::*;
pub fn read_to_u64(vector: &BitVec<u8, Msb0>, bss: &BitVec<u8, Msb0>, mut i: usize) -> (u64, usize) {
    let mut start = i;
    let vr = bss.clone();
    let vrn = to_u64(&vr);
    for _ in 1..vrn {
        i += 1;
    }
    let vc = &vector[start..=i];
    (to_u64(&vc), i + 1)
}
pub fn to_u64(slice: &BitSlice<u8, Msb0>) -> u64 {
    let mut value = 0;
    for bit in slice {
        value = value * 2 + (*bit as u64);
    }
    value
}
pub fn read_to_vec(vector: &BitVec<u8, Msb0>, bss: &BitVec<u8, Msb0>, mut i: usize) -> (BitVec<u8, Msb0>, usize) {                  let mut start = i;     let vr = bss.clone();                         let vrn = to_u64(&vr);                        for _ in 1..vrn {
        i += 1;            }                      let vc = vector[start..=i].to_bitvec();                  (vc, i + 1)                      }
