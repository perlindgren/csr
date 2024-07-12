# CSR

A crate for abstracting RISC-V CSR accesses.

- `csr`, main crate/library providing low-level CSR access
- `csr_derive`, a derive macro for abstracting register blocks
- `csr_trait`, trait for the derive macro

## Resources

- [Volume 1, Unprivileged Specification](https://riscv.org/technical/specifications/), (20240411)

## Use

For now the functionality is limited to low-level CSR access. See the `csr_examples` for usage.

## License

TBD; Likely MIT + Apache
