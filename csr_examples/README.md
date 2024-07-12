# Csr Examples

The `src/main.rs` file covers the complete low-level CSR access macros.

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use csr::*;
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
    let _r = unsafe { csrrwi!(0x100, 0x10) };

    // Read and set with immediate
    let _r = unsafe { csrrsi!(0x100, 0x10) };

    // Read and clear with immediate
    let _r = unsafe { csrrci!(0x100, 0x10) };

    loop {}
}
```

Each CSR operation expands to RV32I assembly as seen by:

```shell
cargo objdump  --release -- -S > csr_ex.objdump
```

```assembly
00000074 <main>:
      74: 73 25 00 10  	csrr	a0, sstatus
      78: 73 10 05 10  	csrw	sstatus, a0
      7c: 73 20 05 10  	csrs	sstatus, a0
      80: 73 30 05 10  	csrc	sstatus, a0
      84: f3 15 05 10  	csrrw	a1, sstatus, a0
      88: 73 a5 05 10  	csrrs	a0, sstatus, a1
      8c: f3 35 05 10  	csrrc	a1, sstatus, a0
      90: 73 50 08 10  	csrwi	sstatus, 0x10
      94: 73 60 08 10  	csrsi	sstatus, 0x10
      98: 73 70 08 10  	csrci	sstatus, 0x10
      9c: 73 55 08 10  	csrrwi	a0, sstatus, 0x10
      a0: 73 65 08 10  	csrrsi	a0, sstatus, 0x10
      a4: 73 75 08 10  	csrrci	a0, sstatus, 0x10
      a8: 6f 00 00 00  	j	0xa8 <main+0x34>
```

The macro expansions can be observed by:

```shell
cargo expand > exp.rs
```

The expanded `exp.rs` will contain:

```rust
...

#[export_name = "main"]
pub fn __risc_v_rt__main() -> ! {
    let r: usize = unsafe {
        {
            let r: usize;
            asm!("csrr {0}, 0x100", out(reg) r);
            r
        }
    };

    ...

    loop {}
}

```
