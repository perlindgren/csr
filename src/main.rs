use csr_derive::CsrDerive;
use csr_macro::read_write;
use csr_trait::Csr;

#[derive(CsrDerive)]
struct Gpio {}

fn main() {
    let g = Gpio {};
    println!("g.read_write {}", g.read_write(10, 0));
    println!("read_write {}", read_write!(10, 0));
}
