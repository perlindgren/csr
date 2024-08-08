#![no_std]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(asm_const)]

use core::intrinsics::is_val_statically_known;
use csr::*;
pub trait CSR: Sized {
    const ADDR: u16;
    fn read(&mut self) -> usize {
        unsafe { csrr!(Self::ADDR) }
    }
    fn write(&mut self, val: u32) {
        write_reg(self, val);
    }
    fn read_write(&mut self, val: u32) -> usize {
        read_write_reg(self, val)
    }

    fn set(&mut self, val: u32) {
        set_reg(self, val);
    }
    fn read_set(&mut self, val: u32) -> usize {
        read_set_reg(self, val)
    }

    fn clear(&mut self, val: u32) {
        clear_reg(self, val);
    }
    fn read_clear(&mut self, val: u32) -> usize {
        read_clear_reg(self, val)
    }
}

trait CSRInternal {
    unsafe fn write_const<const N: u32>(&mut self);
    unsafe fn write_rt(&mut self, val: u32);
    unsafe fn read_write_const<const N: u32>(&mut self) -> usize;
    unsafe fn read_write_rt(&mut self, val: u32) -> usize;

    unsafe fn set_const<const N: u32>(&mut self);
    unsafe fn set_rt(&mut self, val: u32);
    unsafe fn read_set_const<const N: u32>(&mut self) -> usize;
    unsafe fn read_set_rt(&mut self, val: u32) -> usize;

    unsafe fn clear_const<const N: u32>(&mut self);
    unsafe fn clear_rt(&mut self, val: u32);
    unsafe fn read_clear_const<const N: u32>(&mut self) -> usize;
    unsafe fn read_clear_rt(&mut self, val: u32) -> usize;
}

impl<T> CSRInternal for T
where
    T: CSR,
{
    unsafe fn write_const<const N: u32>(&mut self) {
        csrwi!(Self::ADDR, N)
    }
    unsafe fn write_rt(&mut self, val: u32) {
        csrw!(Self::ADDR, val)
    }
    unsafe fn read_write_const<const N: u32>(&mut self) -> usize {
        csrrwi!(Self::ADDR, N)
    }
    unsafe fn read_write_rt(&mut self, val: u32) -> usize {
        csrrw!(Self::ADDR, val)
    }

    unsafe fn set_const<const N: u32>(&mut self) {
        csrsi!(Self::ADDR, N)
    }
    unsafe fn set_rt(&mut self, val: u32) {
        csrs!(Self::ADDR, val)
    }
    unsafe fn read_set_const<const N: u32>(&mut self) -> usize {
        csrrsi!(Self::ADDR, N)
    }
    unsafe fn read_set_rt(&mut self, val: u32) -> usize {
        csrrs!(Self::ADDR, val)
    }

    unsafe fn clear_const<const N: u32>(&mut self) {
        csrci!(Self::ADDR, N)
    }
    unsafe fn clear_rt(&mut self, val: u32) {
        csrc!(Self::ADDR, val)
    }
    unsafe fn read_clear_const<const N: u32>(&mut self) -> usize {
        csrrci!(Self::ADDR, N)
    }
    unsafe fn read_clear_rt(&mut self, val: u32) -> usize {
        csrrc!(Self::ADDR, val)
    }
}
#[inline(always)]
fn write_reg(reg: &mut impl CSR, val: u32) {
    if is_val_statically_known(val) {
        unsafe {
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
        }
    } else {
        unsafe { reg.write_rt(val) }
    }
}
#[inline(always)]
fn read_write_reg(reg: &mut impl CSR, val: u32) -> usize {
    if is_val_statically_known(val) {
        unsafe {
            match val {
                0 => reg.read_write_const::<0>(),
                1 => reg.read_write_const::<1>(),
                2 => reg.read_write_const::<2>(),
                3 => reg.read_write_const::<3>(),
                4 => reg.read_write_const::<4>(),
                5 => reg.read_write_const::<5>(),
                6 => reg.read_write_const::<6>(),
                7 => reg.read_write_const::<7>(),
                8 => reg.read_write_const::<8>(),
                9 => reg.read_write_const::<9>(),
                10 => reg.read_write_const::<10>(),
                11 => reg.read_write_const::<11>(),
                12 => reg.read_write_const::<12>(),
                13 => reg.read_write_const::<13>(),
                14 => reg.read_write_const::<14>(),
                15 => reg.read_write_const::<15>(),
                16 => reg.read_write_const::<16>(),
                17 => reg.read_write_const::<17>(),
                18 => reg.read_write_const::<18>(),
                19 => reg.read_write_const::<19>(),
                20 => reg.read_write_const::<20>(),
                21 => reg.read_write_const::<21>(),
                22 => reg.read_write_const::<22>(),
                23 => reg.read_write_const::<23>(),
                24 => reg.read_write_const::<24>(),
                25 => reg.read_write_const::<25>(),
                26 => reg.read_write_const::<26>(),
                27 => reg.read_write_const::<27>(),
                28 => reg.read_write_const::<28>(),
                29 => reg.read_write_const::<29>(),
                30 => reg.read_write_const::<30>(),
                31 => reg.read_write_const::<31>(),
                // if val is out of immediate range do a regular write
                _ => reg.read_write_rt(val),
            }
        }
    } else {
        unsafe { reg.read_write_rt(val) }
    }
}

#[inline(always)]
fn set_reg(reg: &mut impl CSR, val: u32) {
    if is_val_statically_known(val) {
        unsafe {
            match val {
                0 => reg.set_const::<0>(),
                1 => reg.set_const::<1>(),
                2 => reg.set_const::<2>(),
                3 => reg.set_const::<3>(),
                4 => reg.set_const::<4>(),
                5 => reg.set_const::<5>(),
                6 => reg.set_const::<6>(),
                7 => reg.set_const::<7>(),
                8 => reg.set_const::<8>(),
                9 => reg.set_const::<9>(),
                10 => reg.set_const::<10>(),
                11 => reg.set_const::<11>(),
                12 => reg.set_const::<12>(),
                13 => reg.set_const::<13>(),
                14 => reg.set_const::<14>(),
                15 => reg.set_const::<15>(),
                16 => reg.set_const::<16>(),
                17 => reg.set_const::<17>(),
                18 => reg.set_const::<18>(),
                19 => reg.set_const::<19>(),
                20 => reg.set_const::<20>(),
                21 => reg.set_const::<21>(),
                22 => reg.set_const::<22>(),
                23 => reg.set_const::<23>(),
                24 => reg.set_const::<24>(),
                25 => reg.set_const::<25>(),
                26 => reg.set_const::<26>(),
                27 => reg.set_const::<27>(),
                28 => reg.set_const::<28>(),
                29 => reg.set_const::<29>(),
                30 => reg.set_const::<30>(),
                31 => reg.set_const::<31>(),
                // if val is out of immediate range do a regular write
                _ => reg.set_rt(val),
            }
        }
    } else {
        unsafe { reg.set_rt(val) }
    }
}

#[inline(always)]
fn read_set_reg(reg: &mut impl CSR, val: u32) -> usize {
    if is_val_statically_known(val) {
        unsafe {
            match val {
                0 => reg.read_set_const::<0>(),
                1 => reg.read_set_const::<1>(),
                2 => reg.read_set_const::<2>(),
                3 => reg.read_set_const::<3>(),
                4 => reg.read_set_const::<4>(),
                5 => reg.read_set_const::<5>(),
                6 => reg.read_set_const::<6>(),
                7 => reg.read_set_const::<7>(),
                8 => reg.read_set_const::<8>(),
                9 => reg.read_set_const::<9>(),
                10 => reg.read_set_const::<10>(),
                11 => reg.read_set_const::<11>(),
                12 => reg.read_set_const::<12>(),
                13 => reg.read_set_const::<13>(),
                14 => reg.read_set_const::<14>(),
                15 => reg.read_set_const::<15>(),
                16 => reg.read_set_const::<16>(),
                17 => reg.read_set_const::<17>(),
                18 => reg.read_set_const::<18>(),
                19 => reg.read_set_const::<19>(),
                20 => reg.read_set_const::<20>(),
                21 => reg.read_set_const::<21>(),
                22 => reg.read_set_const::<22>(),
                23 => reg.read_set_const::<23>(),
                24 => reg.read_set_const::<24>(),
                25 => reg.read_set_const::<25>(),
                26 => reg.read_set_const::<26>(),
                27 => reg.read_set_const::<27>(),
                28 => reg.read_set_const::<28>(),
                29 => reg.read_set_const::<29>(),
                30 => reg.read_set_const::<30>(),
                31 => reg.read_set_const::<31>(),
                // if val is out of immediate range do a regular write
                _ => reg.read_set_rt(val),
            }
        }
    } else {
        unsafe { reg.read_set_rt(val) }
    }
}

#[inline(always)]
fn clear_reg(reg: &mut impl CSR, val: u32) {
    if is_val_statically_known(val) {
        unsafe {
            match val {
                0 => reg.clear_const::<0>(),
                1 => reg.clear_const::<1>(),
                2 => reg.clear_const::<2>(),
                3 => reg.clear_const::<3>(),
                4 => reg.clear_const::<4>(),
                5 => reg.clear_const::<5>(),
                6 => reg.clear_const::<6>(),
                7 => reg.clear_const::<7>(),
                8 => reg.clear_const::<8>(),
                9 => reg.clear_const::<9>(),
                10 => reg.clear_const::<10>(),
                11 => reg.clear_const::<11>(),
                12 => reg.clear_const::<12>(),
                13 => reg.clear_const::<13>(),
                14 => reg.clear_const::<14>(),
                15 => reg.clear_const::<15>(),
                16 => reg.clear_const::<16>(),
                17 => reg.clear_const::<17>(),
                18 => reg.clear_const::<18>(),
                19 => reg.clear_const::<19>(),
                20 => reg.clear_const::<20>(),
                21 => reg.clear_const::<21>(),
                22 => reg.clear_const::<22>(),
                23 => reg.clear_const::<23>(),
                24 => reg.clear_const::<24>(),
                25 => reg.clear_const::<25>(),
                26 => reg.clear_const::<26>(),
                27 => reg.clear_const::<27>(),
                28 => reg.clear_const::<28>(),
                29 => reg.clear_const::<29>(),
                30 => reg.clear_const::<30>(),
                31 => reg.clear_const::<31>(),
                // if val is out of immediate range do a regular write
                _ => reg.clear_rt(val),
            }
        }
    } else {
        unsafe { reg.clear_rt(val) }
    }
}

#[inline(always)]
fn read_clear_reg(reg: &mut impl CSR, val: u32) -> usize {
    if is_val_statically_known(val) {
        unsafe {
            match val {
                0 => reg.read_clear_const::<0>(),
                1 => reg.read_clear_const::<1>(),
                2 => reg.read_clear_const::<2>(),
                3 => reg.read_clear_const::<3>(),
                4 => reg.read_clear_const::<4>(),
                5 => reg.read_clear_const::<5>(),
                6 => reg.read_clear_const::<6>(),
                7 => reg.read_clear_const::<7>(),
                8 => reg.read_clear_const::<8>(),
                9 => reg.read_clear_const::<9>(),
                10 => reg.read_clear_const::<10>(),
                11 => reg.read_clear_const::<11>(),
                12 => reg.read_clear_const::<12>(),
                13 => reg.read_clear_const::<13>(),
                14 => reg.read_clear_const::<14>(),
                15 => reg.read_clear_const::<15>(),
                16 => reg.read_clear_const::<16>(),
                17 => reg.read_clear_const::<17>(),
                18 => reg.read_clear_const::<18>(),
                19 => reg.read_clear_const::<19>(),
                20 => reg.read_clear_const::<20>(),
                21 => reg.read_clear_const::<21>(),
                22 => reg.read_clear_const::<22>(),
                23 => reg.read_clear_const::<23>(),
                24 => reg.read_clear_const::<24>(),
                25 => reg.read_clear_const::<25>(),
                26 => reg.read_clear_const::<26>(),
                27 => reg.read_clear_const::<27>(),
                28 => reg.read_clear_const::<28>(),
                29 => reg.read_clear_const::<29>(),
                30 => reg.read_clear_const::<30>(),
                31 => reg.read_clear_const::<31>(),
                // if val is out of immediate range do a regular write
                _ => reg.read_clear_rt(val),
            }
        }
    } else {
        unsafe { reg.read_clear_rt(val) }
    }
}
