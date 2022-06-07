use crate::nes::{ram::Ram, bus::Bus};

pub struct CpuBus<'a, T: Bus> {
    program_rom: &'a Vec<u8>,
    wram: &'a mut Ram,
    ppu_bus: &'a mut T,
}

impl <'a, T> CpuBus<'a, T> where T: Bus {
    pub fn new(program_rom: &'a Vec<u8>, wram: &'a mut Ram, ppu_bus: &'a mut T) -> Self {
        Self { program_rom, wram, ppu_bus }
    }
}

impl <'a, T> Bus for CpuBus<'a, T> where T: Bus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x07FF => *self.wram.read(address),
            0x0800..=0x1FFF => *self.wram.read(address - 0x0800),
            0x2000..=0x3FFF => self.ppu_bus.read(address - 0x2000),
            // 0x4000..0x401F => unimplemented!(), // APU I/O Keypad
            // 0x4020..0x5FFF => unimplemented!(), // Expansion Rom
            // 0x6000..0x7FFF => unimplemented!(), // Expansion Ram
            0x8000..=0xBFFF => *self.program_rom.get((address - 0x8000) as usize).unwrap(),
            0xC000..=0xFFFF if self.program_rom.len() <= 0x4000 => {
                *self.program_rom.get((address - 0xC000) as usize).unwrap()
            },
            0xC000..=0xFFFF => *self.program_rom.get((address - 0x8000) as usize).unwrap(),
            // _ => panic!("unexpected memory area access!"),
            _ => panic!("unexpected memory area access! Addr: {:x}", address),

        }
    }

    fn write(&mut self, address: u16, data: u8) {
        println!("CpuBus write Address {:x} / Data: {:x}", &address, &data);
        match address {
            0x0000..=0x07FF => self.wram.write(address, data.into()),
            0x0800..=0x1FFF => self.wram.write(address - 0x0800, data.into()),
            0x2000..=0x3FFF => self.ppu_bus.write(address - 0x2000, data),
            // 0x4000..0x401F => unimplemented!(), // APU I/O Keypad
            // 0x4020..0x5FFF => unimplemented!(), // Expansion Rom
            // 0x6000..0x7FFF => unimplemented!(), // Expansion Ram
            _ => panic!("unexpected memory area access! Addr: {:x} / Data: {:x}", address, data),
        };
    }
}
