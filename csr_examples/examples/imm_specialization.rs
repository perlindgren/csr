#![no_std]
#![no_main]
#![feature(asm_const)]
#![allow(internal_features)]
#![feature(core_intrinsics)]

use core::{intrinsics::is_val_statically_known, marker::PhantomData, panic::PanicInfo};
use csr::*;
use hippomenes_rt::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
mod util {
    use core::intrinsics::is_val_statically_known;

    use super::CSR;
    #[inline(always)]
    pub fn write_reg(reg: &mut impl CSR, val: u32) {
        if is_val_statically_known(val) {
            match val {
                0 => reg.write_const::<0>(),
                1 => reg.write_const::<1>(),
                2 => reg.write_const::<2>(),
                3 => reg.write_const::<3>(),
                4 => reg.write_const::<4>(),
                5 => reg.write_const::<5>(),
                6 => reg.write_const::<6>(),
                7 => reg.write_const::<7>(),
                8 => reg.write_const::<8>(),
                9 => reg.write_const::<9>(),
                10 => reg.write_const::<10>(),
                11 => reg.write_const::<11>(),
                12 => reg.write_const::<12>(),
                13 => reg.write_const::<13>(),
                14 => reg.write_const::<14>(),
                15 => reg.write_const::<15>(),
                16 => reg.write_const::<16>(),
                17 => reg.write_const::<17>(),
                18 => reg.write_const::<18>(),
                19 => reg.write_const::<19>(),
                20 => reg.write_const::<20>(),
                21 => reg.write_const::<21>(),
                22 => reg.write_const::<22>(),
                23 => reg.write_const::<23>(),
                24 => reg.write_const::<24>(),
                25 => reg.write_const::<25>(),
                26 => reg.write_const::<26>(),
                27 => reg.write_const::<27>(),
                28 => reg.write_const::<28>(),
                29 => reg.write_const::<29>(),
                30 => reg.write_const::<30>(),
                31 => reg.write_const::<31>(),
                // if val is out of immediate range do a regular write
                _ => reg.write_rt(val),
            }
        } else {
            reg.write_rt(val)
        }
    }
}
// implements static vs dynamic specialization
macro_rules! register_write {
    ($register:expr, $val: expr) => {{
        {
            if is_val_statically_known($val) {
                // since val is statically known this should be folded.
                // i think it looks stupid but it does work better than anything else i've tried
                // so for now it stays
                match $val {
                    0 => $register.write_const::<0>(),
                    1 => $register.write_const::<1>(),
                    2 => $register.write_const::<2>(),
                    3 => $register.write_const::<3>(),
                    4 => $register.write_const::<4>(),
                    5 => $register.write_const::<5>(),
                    6 => $register.write_const::<6>(),
                    7 => $register.write_const::<7>(),
                    8 => $register.write_const::<8>(),
                    9 => $register.write_const::<9>(),
                    10 => $register.write_const::<10>(),
                    11 => $register.write_const::<11>(),
                    12 => $register.write_const::<12>(),
                    13 => $register.write_const::<13>(),
                    14 => $register.write_const::<14>(),
                    15 => $register.write_const::<15>(),
                    16 => $register.write_const::<16>(),
                    17 => $register.write_const::<17>(),
                    18 => $register.write_const::<18>(),
                    19 => $register.write_const::<19>(),
                    20 => $register.write_const::<20>(),
                    21 => $register.write_const::<21>(),
                    22 => $register.write_const::<22>(),
                    23 => $register.write_const::<23>(),
                    24 => $register.write_const::<24>(),
                    25 => $register.write_const::<25>(),
                    26 => $register.write_const::<26>(),
                    27 => $register.write_const::<27>(),
                    28 => $register.write_const::<28>(),
                    29 => $register.write_const::<29>(),
                    30 => $register.write_const::<30>(),
                    31 => $register.write_const::<31>(),
                    // if val is out of immediate range do a regular write
                    _ => $register.write_rt($val),
                }
            } else {
                $register.write($val)
            }
        }
    }};
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
trait CSR {
    const ADDR: u16;
    fn write_const<const N: u32>(&mut self);
    fn write_rt(&mut self, val: u32);
}
/// This should probably be a Register trait implementation. This will be generated.
impl CSR for SomeReg {
    const ADDR: u16 = 0x100;

    /// Writes to the register via immediate instruction.
    fn write_const<const N: u32>(&mut self) {
        unsafe { csrwi!(SomeReg::ADDR, N) };
    }
    /// Writes to the register via non-immediate instruction.
    // todo: change the csrrw! macro to accept a const address
    fn write_rt(&mut self, val: u32) {
        let val = val as i32;
        unsafe { csrrw!(0x100, val) };
    }
}

impl SomeReg {
    fn write(&mut self, val: u32) {
        util::write_reg(self, val);
    }
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
    register_write!(p.some_peripheral.some_reg, 1);
    register_write!(p.some_peripheral.some_reg, x);
    register_write!(p.some_peripheral.some_reg, X);
    register_write!(p.some_peripheral.some_reg, gen_x());

    // all of these are statically known but out of range so should end up as non-immediate
    register_write!(p.some_peripheral.some_reg, 32);
    register_write!(p.some_peripheral.some_reg, y);
    register_write!(p.some_peripheral.some_reg, gen_y());

    let z = unsafe { core::ptr::read(0x13370000 as *const u32) };
    // z is unknown so this should end up as non-immediate
    register_write!(p.some_peripheral.some_reg, z);

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

    loop {}
}
