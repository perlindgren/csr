#![no_std]
#![no_main]

use core::{marker::PhantomData, panic::PanicInfo};
use csr_trait::CSR;
use hippomenes_rt::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Dummy Peripherals. This will be generated.
struct Peripherals {
    pub some_peripheral: SomePeripheral,
}

/// Some dummy peripheral. This will be generated.
struct SomePeripheral {
    pub some_reg: SomeReg,
}

/// Some dummy register of the dummy peripheral. This will be generated
struct SomeReg {
    _marker: PhantomData<*const ()>,
}
/// Implementation of the CSR trait. This will be generated.
impl CSR for SomeReg {
    const ADDR: u16 = 0x100;
}
/// Standard Peripherals fns (steal, take, etc.). This will be generated
#[allow(non_snake_case)]
impl Peripherals {
    unsafe fn steal() -> Peripherals {
        let some_reg: SomeReg = SomeReg {
            _marker: PhantomData,
        };
        let some_peripheral: SomePeripheral = SomePeripheral { some_reg };
        Peripherals { some_peripheral }
    }
    // TODO: implement rest of Peripherals.
}
const X: u32 = 3;
const fn gen_x() -> u32 {
    4
}
const fn gen_y() -> u32 {
    33
}

#[entry]
fn main() -> ! {
    let mut p: Peripherals = unsafe { Peripherals::steal() };
    let x: u32 = 2;
    let y: u32 = 32;

    // all of these are statically known so should end up as immediate
    p.some_peripheral.some_reg.write(1);
    p.some_peripheral.some_reg.write(x);
    p.some_peripheral.some_reg.write(X);
    p.some_peripheral.some_reg.write(gen_x());

    // all of these are statically known but out of range so should end up as non-immediate
    p.some_peripheral.some_reg.write(32);
    p.some_peripheral.some_reg.write(y);
    p.some_peripheral.some_reg.write(gen_y());

    let z = unsafe { core::ptr::read(0x13370000 as *const u32) };
    // z is unknown so this should end up as non-immediate
    p.some_peripheral.some_reg.write(z);

    // all of these are statically known so should end up as immediate
    p.some_peripheral.some_reg.set(1);
    p.some_peripheral.some_reg.set(x);
    p.some_peripheral.some_reg.set(X);
    p.some_peripheral.some_reg.set(gen_x());

    // all of these are statically known but out of range so should end up as non-immediate
    p.some_peripheral.some_reg.set(32);
    p.some_peripheral.some_reg.set(y);
    p.some_peripheral.some_reg.set(gen_y());

    // z is unknown so this should end up as non-immediate
    p.some_peripheral.some_reg.set(z);

    // all of these are statically known so should end up as immediate
    p.some_peripheral.some_reg.clear(1);
    p.some_peripheral.some_reg.clear(x);
    p.some_peripheral.some_reg.clear(X);
    p.some_peripheral.some_reg.clear(gen_x());

    // all of these are statically known but out of range so should end up as non-immediate
    p.some_peripheral.some_reg.clear(32);
    p.some_peripheral.some_reg.clear(y);
    p.some_peripheral.some_reg.clear(gen_y());

    // z is unknown so this should end up as non-immediate
    p.some_peripheral.some_reg.clear(z);

    // all of these are statically known so should end up as immediate
    p.some_peripheral.some_reg.read_write(1);
    p.some_peripheral.some_reg.read_write(x);
    p.some_peripheral.some_reg.read_write(X);
    p.some_peripheral.some_reg.read_write(gen_x());

    // all of these are statically known but out of range so should end up as non-immediate
    p.some_peripheral.some_reg.read_write(32);
    p.some_peripheral.some_reg.read_write(y);
    p.some_peripheral.some_reg.read_write(gen_y());

    // z is unknown so this should end up as non-immediate
    p.some_peripheral.some_reg.read_write(z);

    // all of these are statically known so should end up as immediate
    p.some_peripheral.some_reg.read_set(1);
    p.some_peripheral.some_reg.read_set(x);
    p.some_peripheral.some_reg.read_set(X);
    p.some_peripheral.some_reg.read_set(gen_x());

    // all of these are statically known but out of range so should end up as non-immediate
    p.some_peripheral.some_reg.read_set(32);
    p.some_peripheral.some_reg.read_set(y);
    p.some_peripheral.some_reg.read_set(gen_y());

    // z is unknown so this should end up as non-immediate
    p.some_peripheral.some_reg.read_set(z);

    // all of these are statically known so should end up as immediate
    p.some_peripheral.some_reg.read_clear(1);
    p.some_peripheral.some_reg.read_clear(x);
    p.some_peripheral.some_reg.read_clear(X);
    p.some_peripheral.some_reg.read_clear(gen_x());

    // all of these are statically known but out of range so should end up as non-immediate
    p.some_peripheral.some_reg.read_clear(32);
    p.some_peripheral.some_reg.read_clear(y);
    p.some_peripheral.some_reg.read_clear(gen_y());

    // z is unknown so this should end up as non-immediate
    p.some_peripheral.some_reg.read_clear(z);

    loop {}
}
