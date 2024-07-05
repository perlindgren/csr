// #![allow(unused_variables)]
use csr_macro::CSRRW;

fn main() {
    let r: usize = 10;
    let _ = unsafe { CSRRW!(0x100, r) };
}
