#![no_std]
#![feature(asm_const)]

/// Low-Level CSR access macros:
/// See latest: https://riscv.org/technical/specifications/
/// Implementation based on Volume 1, Unprivileged Specification
///
#[macro_export]
macro_rules! csrr {
    ($csr_number:expr) => {
        /// Reads the CSR
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrr {0}, {csr}"),
                out(reg) r,
                csr = const $csr_number
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrw {
    ($csr_number:expr, $write_register: ident) => {
        /// Writes the CSR with register
        {
            core::arch::asm!(
                "csrw {csr}, {0}", in(reg) $write_register, csr = const $csr_number
            );
        }
    };
}

#[macro_export]
macro_rules! csrrw {
    ($csr_number:expr, $read_register: ident) => {
        /// Reads and clears the CSR
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrw {0}, {csr}, {1}"),
                out(reg) r,
                in(reg) $read_register,
                csr = const $csr_number
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrs {
    ($csr_number:expr, $read_register: ident) => {
        /// Sets the CSR
        {
            core::arch::asm!(
                "csrs {csr}, {0}",
                in(reg) $read_register,
                csr = const $csr_number

            );
        }
    };
}

#[macro_export]
macro_rules! csrc {
    ($csr_number:expr, $read_register: ident) => {
        /// Sets the CSR
        {
            core::arch::asm!(
                "csrc {csr}, {0}",
                in(reg) $read_register,
                csr = const $csr_number

            );
        }
    };
}

#[macro_export]
macro_rules! csrrs {
    ($csr_number:expr, $read_register: ident) => {
        /// Reads and clears the CSR
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrs {0}, {csr}, {1}"),
                out(reg) r,
                in(reg) $read_register,
                csr = const $csr_number
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrrc {
    ($csr_number:expr, $read_register: ident) => {
        /// Reads and clears the CSR
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrc {0}, {csr}, {1}"),
                out(reg) r,
                in(reg) $read_register,
                csr = const $csr_number
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrwi {
    ($csr_number:expr, $immediate: expr) => {
        /// Writes the CSR with immediate
        {
            core::arch::asm!(
                concat!(
                    "csrwi {csr}, {imm}"
                ),
                imm = const $immediate,
                csr = const $csr_number
            );
        }
    };
}

#[macro_export]
macro_rules! csrrwi {
    ($csr_number:expr, $immediate: expr) => {
        /// Reads and writes the CSR with immediate
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrwi {0}, {csr}, {imm}"),
                out(reg) r,
                imm = const $immediate,
                csr = const $csr_number
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrci {
    ($csr_number:expr, $immediate: expr) => {
        /// Writes the CSR with immediate
        {
            core::arch::asm!(
                concat!(
                    "csrci {csr}, {imm}"
                ),
                imm = const $immediate,
                csr = const $csr_number
            );
        }
    };
}

#[macro_export]
macro_rules! csrrci {
    ($csr_number:expr, $immediate: expr) => {
        /// Reads and writes the CSR with immediate
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrci {0}, {csr}, {imm}"),
                out(reg) r,
                imm = const $immediate,
                csr = const $csr_number
            );
            r
        }
    };
}

#[macro_export]
macro_rules! csrsi {
    ($csr_number:expr, $immediate: expr) => {
        /// Writes the CSR with immediate
        {
            core::arch::asm!(
                concat!(
                    "csrsi {csr}, {imm}"
                ),
                imm = const $immediate,
                csr = const $csr_number
            );
        }
    };
}

#[macro_export]
macro_rules! csrrsi {
    ($csr_number:expr, $immediate: expr) => {
        /// Reads and writes the CSR with immediate
        {
            let r: usize;
            core::arch::asm!(
                concat!("csrrsi {0}, {csr}, {imm}"),
                out(reg) r,
                imm = const $immediate,
                csr = const $csr_number
            );
            r
        }
    };
}
