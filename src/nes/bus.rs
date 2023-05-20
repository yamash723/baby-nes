pub trait Bus {
    fn read(&mut self, address: u16) -> u8;
    fn read_u16(&mut self, address: u16) -> u16;
    fn write(&mut self, address: u16, data: u8);
}
