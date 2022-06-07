use crate::nes::ram::Ram;

pub struct PpuContext<'a> {
    pub pattern_table: &'a mut Vec<u8>,
    pub vram: &'a mut Ram,
    pub palette_ram: &'a mut Ram,
}