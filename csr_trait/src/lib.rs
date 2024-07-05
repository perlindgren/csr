pub trait Csr {
    fn read_write(&self, addr: u16, val: u8) -> u32;
}
