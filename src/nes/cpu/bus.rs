use crate::nes::{ram::Ram, bus::Bus};

pub struct CpuBus<'a> {
    program_rom: &'a Vec<u8>,
    wram: &'a mut Ram,
}

impl <'a> CpuBus<'a> {
    pub fn new(program_rom: &'a Vec<u8>, wram: &'a mut Ram) -> Self {
        Self { program_rom, wram }
    }
}

impl Bus for CpuBus<'_> {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x07FF => *self.wram.read(address),
            0x0800..=0x1FFF => *self.wram.read(address - 0x0800),
            // 0x2000..=0x3FFF => self.ppu.read(address - 0x2000),
            // 0x4000..0x401F => unimplemented!(), // APU I/O Keypad
            // 0x4020..0x5FFF => unimplemented!(), // Expansion Rom
            // 0x6000..0x7FFF => unimplemented!(), // Expansion Ram
            0x8000..=0xBFFF => *self.program_rom.get((address - 0x8000) as usize).unwrap(),
            0xC000..=0xFFFF if self.program_rom.len() <= 0x4000 => {
                *self.program_rom.get((address - 0xC000) as usize).unwrap()
            },
            0xC000..=0xFFFF => *self.program_rom.get((address - 0x8000) as usize).unwrap(),
            // _ => panic!("unexpected memory area access!"),
            _ => {
                println!("unexpected memory area access! Addr: {:x}", address);
                0
            },

        }
    }

    fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x07FF => self.wram.write(address, data.into()),
            0x0800..=0x1FFF => self.wram.write(address - 0x0800, data.into()),
            // 0x2000..=0x3FFF => self.ppu.write(address - 0x2000, data),
            // 0x4000..0x401F => unimplemented!(), // APU I/O Keypad
            // 0x4020..0x5FFF => unimplemented!(), // Expansion Rom
            // 0x6000..0x7FFF => unimplemented!(), // Expansion Ram
            // _ => panic!("unexpected memory area access! Addr: {:x}", address),
            _ => {
                println!("unexpected memory area access! Addr: {:x}", address);
                self.wram.write(0, data.into())
            },
        };
    }
}
