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
cargo objdump --release --example low_level_access -- -d > low_level_access.objdump
```

```assembly
00000074 <main>:
      74: 10002573     	csrr	a0, sstatus
      78: 10051073     	csrw	sstatus, a0
      7c: 10052073     	csrs	sstatus, a0
      80: 10053073     	csrc	sstatus, a0
      84: 100515f3     	csrrw	a1, sstatus, a0
      88: 1005a573     	csrrs	a0, sstatus, a1
      8c: 100535f3     	csrrc	a1, sstatus, a0
      90: 10085073     	csrwi	sstatus, 0x10
      94: 1002d073     	csrwi	sstatus, 0x5
      98: 1002d073     	csrwi	sstatus, 0x5
      9c: 10086073     	csrsi	sstatus, 0x10
      a0: 10087073     	csrci	sstatus, 0x10
      a4: 10085573     	csrrwi	a0, sstatus, 0x10
      a8: 10086573     	csrrsi	a0, sstatus, 0x10
      ac: 10087573     	csrrci	a0, sstatus, 0x10
      b0: 0000006f     	j	0xb0 <main+0x3c>
```

The macro expansions can be observed by:

```shell
cargo expand --example low_level_access > exp_low_level_acccess.rs
```

The expanded `exp_low_level_access.rs` will contain:

```rust
...

#[export_name = "main"]
pub fn __risc_v_rt__main() -> ! {
    let r: usize = unsafe {
        {
            let r: usize;
            asm!("csrr {0}, {1}", out(reg) r, const 0x100);
            r
        }
    };

    ...

    loop {}
}

```
