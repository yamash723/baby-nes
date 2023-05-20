use crate::nes::ram::Ram;

pub struct Oam {
    addr: u8,
}

impl Oam {
    pub fn new() -> Self {
        Oam {
            addr: 0,
        }
    }

    pub fn read<'a>(&self, sprite_ram: &'a Ram) -> &'a u8 {
        sprite_ram.read(self.addr as u16)
    }

    pub fn write(&mut self, data: u8) {
        self.addr = data;
    }
}
