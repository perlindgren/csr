#![no_std]
#![no_main]

use core::panic::PanicInfo;
use csr_examples::*;
use hippomenes_rt::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
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
    let r = unsafe { csrrc!(0x100, r) };

    // Write with immediate
    unsafe { csrwi!(0x100, 0x10) };

    // Set with immediate
    unsafe { csrsi!(0x100, 0x10) };

    // Clear with immediate
    unsafe { csrci!(0x100, 0x10) };

    // Read and write with immediate
    let r = unsafe { csrrwi!(0x100, 0x10) };

    // Read and set with immediate
    let r = unsafe { csrrsi!(0x100, 0x10) };

    // Read and clear with immediate
    let r = unsafe { csrrci!(0x100, 0x10) };

    loop {}
}
