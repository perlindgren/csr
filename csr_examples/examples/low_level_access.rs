#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;
use csr::*;
use hippomenes_rt::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const X: u8 = 5;
const fn gen_x() -> u8 {
    5
}

#[entry]
fn main() -> ! {
    // Read CSR
    let r: usize = unsafe { csrr!(0x100) };

    // Write CSR
    unsafe { csrw!(0x100, r) };

    // Set CSR with register
    unsafe { csrs!(0x100, r) };

    // Clear CSR with register
    unsafe { csrc!(0x100, r) };

    // Read and write CSR
    let r = unsafe { csrrw!(0x100, r) };

    // Read and set CSR with register
    let r = unsafe { csrrs!(0x100, r) };

    // Read and clear CSR with register
    let _ = unsafe { csrrc!(0x100, r) };

    // Write with immediate
    unsafe { csrwi!(0x100, 0x10) };
    unsafe { csrwi!(0x100, X) };
    unsafe { csrwi!(0x100, gen_x()) };

    // Set with immediate
    unsafe { csrsi!(0x100, 0x10) };

    // Clear with immediate
    unsafe { csrci!(0x100, 0x10) };

    // Read and write with immediate
    let _r = unsafe { csrrwi!(0x100, 0x10) };

    // Read and set with immediate
    let _r = unsafe { csrrsi!(0x100, 0x10) };

    // Read and clear with immediate
    let _r = unsafe { csrrci!(0x100, 0x10) };

    loop {}
}
