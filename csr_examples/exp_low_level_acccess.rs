#![feature(prelude_import)]
#![no_std]
#![no_main]
#![feature(asm_const)]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
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
#[allow(non_snake_case)]
#[export_name = "main"]
pub fn __risc_v_rt__main() -> ! {
    let r: usize = unsafe {
        {
            let r: usize;
            asm!("csrr {0}, {1}", out(reg) r, const 0x100);
            r
        }
    };
    unsafe {
        {
            asm!("csrw {1}, {0}", in (reg) r, const 0x100);
        }
    };
    unsafe {
        {
            asm!("csrs {1}, {0}", in (reg) r, const 0x100);
        }
    };
    unsafe {
        {
            asm!("csrc {1}, {0}", in (reg) r, const 0x100);
        }
    };
    let r = unsafe {
        {
            let r: usize;
            asm!("csrrw {0}, {2}, {1}", out(reg) r, in (reg) r, const 0x100);
            r
        }
    };
    let r = unsafe {
        {
            let r: usize;
            asm!("csrrs {0}, {2}, {1}", out(reg) r, in (reg) r, const 0x100);
            r
        }
    };
    let _ = unsafe {
        {
            let r: usize;
            asm!("csrrc {0}, {2}, {1}", out(reg) r, in (reg) r, const 0x100);
            r
        }
    };
    unsafe {
        {
            asm!("csrwi {1}, {0}", const 0x10, const 0x100);
        }
    };
    unsafe {
        {
            asm!("csrwi {1}, {0}", const X, const 0x100);
        }
    };
    unsafe {
        {
            asm!("csrwi {1}, {0}", const gen_x(), const 0x100);
        }
    };
    unsafe {
        {
            asm!("csrsi {1}, {0}", const 0x10, const 0x100);
        }
    };
    unsafe {
        {
            asm!("csrci {1}, {0}", const 0x10, const 0x100);
        }
    };
    let _r = unsafe {
        {
            let r: usize;
            asm!("csrrwi {0}, {2}, {1}", out(reg) r, const 0x10, const 0x100);
            r
        }
    };
    let _r = unsafe {
        {
            let r: usize;
            asm!("csrrsi {0}, {2}, {1}", out(reg) r, const 0x10, const 0x100);
            r
        }
    };
    let _r = unsafe {
        {
            let r: usize;
            asm!("csrrci {0}, {2}, {1}", out(reg) r, const 0x10, const 0x100);
            r
        }
    };
    loop {}
}
