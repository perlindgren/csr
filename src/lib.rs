#![no_std]
#![feature(asm_const)]

/// Low-Level CSR access macros:
/// See latest: https://riscv.org/technical/specifications/
/// Implementation based on Volume 1, Unprivileged Specification
///
#[macro_export]
macro_rules! csrr {
    ($csr_number:literal) => {
        /// Reads the CSR
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrr {0}, ", stringify!($csr_number)),
                out(reg) r
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrw {
    ($csr_number:literal, $read_register: ident) => {
        /// Writes the CSR with register
        {
            core::arch::asm!(concat!("csrw ", stringify!($csr_number), ", {}"), in(reg) $read_register);
        }
    };
}

#[macro_export]
macro_rules! csrrw {
    ($csr_number:literal, $read_register: ident) => {
        /// Reads and writes the CSR
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrw {0}, ", stringify!($csr_number), ", {1}"),
                out(reg) r,
                in(reg) $read_register
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrs {
    ($csr_number:literal, $read_register: ident) => {
        /// Sets the CSR
        {
            core::arch::asm!(
                concat!("csrs ", stringify!($csr_number), ", {0}"),
                in(reg) $read_register
            );
        }
    };
}

#[macro_export]
macro_rules! csrc {
    ($csr_number:literal, $read_register: ident) => {
        /// Clears the CSR
        {
            core::arch::asm!(
                concat!("csrc ", stringify!($csr_number), ", {0}"),
                in(reg) $read_register
            );
        }
    };
}

#[macro_export]
macro_rules! csrrs {
    ($csr_number:literal, $read_register: ident) => {
        /// Reads and sets the CSR
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrs {0}, ", stringify!($csr_number), ", {1}"),
                out(reg) r,
                in(reg) $read_register
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrrc {
    ($csr_number:literal, $read_register: ident) => {
        /// Reads and clears the CSR
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrc {0}, ", stringify!($csr_number), ", {1}"),
                out(reg) r,
                in(reg) $read_register
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrwi {
    ($csr_number:literal, $immediate: expr) => {
        /// Writes the CSR with immediate
        {
            core::arch::asm!(
                concat!(
                    "csrwi ",
                    stringify!($csr_number),
                    ", {imm}"
                ),
                imm = const $immediate
            );
        }
    };
}

#[macro_export]
macro_rules! csrrwi {
    ($csr_number:literal, $immediate: expr) => {
        /// Reads and writes the CSR with immediate
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrwi {0}, ", stringify!($csr_number), ", ", stringify!($immediate)),
                out(reg) r
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrci {
    ($csr_number:literal, $immediate: expr) => {
        /// Clears the CSR with immediate
        {
            core::arch::asm!(concat!(
                "csrci ",
                stringify!($csr_number),
                ", ",
                stringify!($immediate)
            ));
        }
    };
}

#[macro_export]
macro_rules! csrrci {
    ($csr_number:literal, $immediate: expr) => {
        /// Reads and clears the CSR with immediate
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrci {0}, ", stringify!($csr_number), ", ", stringify!($immediate)),
                out(reg) r
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrsi {
    ($csr_number:literal, $immediate: expr) => {
        /// Sets the CSR with immediate
        {
            core::arch::asm!(concat!(
                "csrsi ",
                stringify!($csr_number),
                ", ",
                stringify!($immediate)
            ));
        }
    };
}

#[macro_export]
macro_rules! csrrsi {
    ($csr_number:literal, $immediate: expr) => {
        /// Reads and sets the CSR with immediate
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrsi {0}, ", stringify!($csr_number), ", ", stringify!($immediate)),
                out(reg) r
            );
            r
        }
    };
}
