use crate::nes::{bus::Bus, ppu::registers::PpuRegistration, ram::Ram};

pub struct CpuBus<'a, T: PpuRegistration> {
    program_rom: &'a Vec<u8>,
    wram: &'a mut Ram,
    ppu: &'a mut T,
}

impl<'a, T> CpuBus<'a, T>
where
    T: PpuRegistration,
{
    pub fn new(program_rom: &'a Vec<u8>, wram: &'a mut Ram, ppu: &'a mut T) -> Self {
        Self {
            program_rom,
            wram,
            ppu,
        }
    }
}

impl<'a, T> Bus for CpuBus<'a, T>
where
    T: PpuRegistration,
{
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => {
                // 0x0000..=0x07FF => access to RAM.
                // 0x0800..=0x0FFF => Mirrors of 0x0000..=0x07FF
                // 0x1000..=0x17FF => Mirrors of 0x0000..=0x07FF
                // 0x1800..=0x1FFF => Mirrors of 0x0000..=0x07FF
                let calibrated_address = address % 0x0800;
                *self.wram.read(calibrated_address)
            }
            0x2000..=0x3FFF => {
                // 0x2000..=0x2007 => access to PPU registers.
                // 0x2008..=0x3FFF => Mirror of 0x2000..=0x2007  every 8 bytes.
                let calibrated_address = (address - 0x2000) % 8;
                self.ppu.read(calibrated_address)
            }
            // 0x4000..0x401F => unimplemented!(), // APU I/O Keypad
            // 0x4020..0x5FFF => unimplemented!(), // Expansion Rom
            // 0x6000..0x7FFF => unimplemented!(), // Expansion Ram
            0x8000..=0xBFFF => *self.program_rom.get((address - 0x8000) as usize).unwrap(),
            0xC000..=0xFFFF => {
                // if program rom sie is 16kb, mirror 0x8000..=0xBFFF
                let calibrated_address = if self.program_rom.len() <= 0x4000 {
                    // Mirror
                    address - 0xC000
                } else {
                    // No mirror
                    address - 0x8000
                };

                *self.program_rom.get(calibrated_address as usize).unwrap()
            }
            _ => panic!("unexpected memory area access! Addr: {:x}", address),
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        println!("CpuBus write Address {:x} / Data: {:x}", &address, &data);
        match address {
            0x0000..=0x1FFF => {
                // 0x0000..=0x07FF => access to RAM.
                // 0x0800..=0x0FFF => Mirrors of 0x0000..=0x07FF
                // 0x1000..=0x17FF => Mirrors of 0x0000..=0x07FF
                // 0x1800..=0x1FFF => Mirrors of 0x0000..=0x07FF
                let calibrated_address = address % 0x0800;
                self.wram.write(calibrated_address, data.into());
            }
            0x2000..=0x3FFF => {
                // 0x2000..=0x2007 => access to PPU registers.
                // 0x2008..=0x3FFF => Mirror of 0x2000..=0x2007  every 8 bytes.
                let calibrated_address = (address - 0x2000) % 8;
                self.ppu.write(calibrated_address, data);
            }
            // 0x4000..0x401F => unimplemented!(), // APU I/O Keypad
            // 0x4020..0x5FFF => unimplemented!(), // Expansion Rom
            // 0x6000..0x7FFF => unimplemented!(), // Expansion Ram
            _ => panic!(
                "unexpected memory area access! Addr: {:x} / Data: {:x}",
                address, data
            ),
        };
    }
}

#[cfg(test)]
mod cpu_bus_test {
    use crate::nes::ppu::registers::PpuRegistration;

    struct MockPpu {
        pub data: Vec<u8>,
    }

    impl MockPpu {
        fn new() -> Self {
            Self {
                data: vec![0; 0x0008],
            }
        }
    }

    impl PpuRegistration for MockPpu {
        fn read(&self, address: u16) -> u8 {
            self.data[address as usize]
        }

        fn write(&mut self, address: u16, data: u8) {
            self.data[address as usize] = data
        }
    }

    mod read_test {
        use crate::nes::{bus::Bus, cpu::bus::CpuBus, ppu::registers::PpuRegistration, ram::Ram};

        #[test]
        fn vram_range_read_test() {
            let program_rom = Vec::new();
            let mut ppu = super::MockPpu::new();

            let mut wram = Ram::new(0x0800);
            wram.write(0x0000, 0x01);
            wram.write(0x07FF, 0x02);

            let bus = CpuBus::new(&program_rom, &mut wram, &mut ppu);

            // Read RAM
            assert_eq!(bus.read(0x0000), 0x01);
            assert_eq!(bus.read(0x07FF), 0x02);
            // Mirror 1
            assert_eq!(bus.read(0x0800), 0x01);
            assert_eq!(bus.read(0x0FFF), 0x02);
            // Mirror 2
            assert_eq!(bus.read(0x1000), 0x01);
            assert_eq!(bus.read(0x17FF), 0x02);
            // Mirror 3
            assert_eq!(bus.read(0x1800), 0x01);
            assert_eq!(bus.read(0x1FFF), 0x02);
        }

        #[test]
        fn ppu_range_read_test() {
            let program_rom = Vec::new();
            let mut wram = Ram::new(0x0800);

            let mut ppu = super::MockPpu::new();
            ppu.write(0x0000, 0x01);
            ppu.write(0x0007, 0x02);

            let bus = CpuBus::new(&program_rom, &mut wram, &mut ppu);

            // Read PPU
            assert_eq!(bus.read(0x2000), 0x01);
            assert_eq!(bus.read(0x2007), 0x02);

            // Mirror every 8 bytes
            // ref : https://www.nesdev.org/wiki/CPU_memory_map
            let mirror_count = (0x3FFF - 0x2008) / 8;
            for i in 1..=mirror_count {
                let offset = i * 8;
                assert_eq!(bus.read(0x2000 + offset), 0x01);
                assert_eq!(bus.read(0x2007 + offset), 0x02);
            }
        }

        #[test]
        fn program_rom_range_read_test() {
            let mut ppu = super::MockPpu::new();
            let mut wram = Ram::new(0x0800);

            let mut program_rom = vec![0x00; 0x8000];
            program_rom[0x0000] = 0x01;
            program_rom[0x3FFF] = 0x02;
            program_rom[0x4000] = 0x03;
            program_rom[0x7FFF] = 0x04;

            let bus = CpuBus::new(&program_rom, &mut wram, &mut ppu);

            // Read Program ROM
            assert_eq!(bus.read(0x8000), 0x01);
            assert_eq!(bus.read(0xBFFF), 0x02);
            assert_eq!(bus.read(0xC000), 0x03);
            assert_eq!(bus.read(0xFFFF), 0x04);
        }

        #[test]
        fn program_rom_range_16kb_rom_read_test() {
            let mut ppu = super::MockPpu::new();
            let mut wram = Ram::new(0x0800);

            let mut program_rom = vec![0x00; 0x4000];
            program_rom[0x0000] = 0x01;
            program_rom[0x3FFF] = 0x02;

            let bus = CpuBus::new(&program_rom, &mut wram, &mut ppu);

            // Read Program ROM
            assert_eq!(bus.read(0x8000), 0x01);
            assert_eq!(bus.read(0xBFFF), 0x02);
            // Mirror
            assert_eq!(bus.read(0xC000), 0x01);
            assert_eq!(bus.read(0xFFFF), 0x02);
        }
    }

    mod write_test {
        use crate::nes::{bus::Bus, cpu::bus::CpuBus, ram::Ram};

        #[test]
        fn vram_range_write_test() {
            let program_rom = Vec::new();
            let mut ppu = super::MockPpu::new();
            let mut wram = Ram::new(0x0800);

            let mut bus = CpuBus::new(&program_rom, &mut wram, &mut ppu);

            // Write RAM
            bus.write(0x0000, 0x01);
            bus.write(0x07FF, 0x02);
            assert_eq!(bus.wram.read(0x0000), &0x01);
            assert_eq!(bus.wram.read(0x07FF), &0x02);
            // Mirror 1
            bus.write(0x0800, 0x03);
            bus.write(0x0FFF, 0x04);
            assert_eq!(bus.wram.read(0x0000), &0x03);
            assert_eq!(bus.wram.read(0x07FF), &0x04);
            // Mirror 2
            bus.write(0x1000, 0x05);
            bus.write(0x17FF, 0x06);
            assert_eq!(bus.wram.read(0x0000), &0x05);
            assert_eq!(bus.wram.read(0x07FF), &0x06);
            // Mirror 3
            bus.write(0x1800, 0x07);
            bus.write(0x1FFF, 0x08);
            assert_eq!(bus.wram.read(0x0000), &0x07);
            assert_eq!(bus.wram.read(0x07FF), &0x08);
        }

        #[test]
        fn ppu_range_write_test() {
            let program_rom = Vec::new();
            let mut wram = Ram::new(0x0800);
            let mut ppu = super::MockPpu::new();

            let mut bus = CpuBus::new(&program_rom, &mut wram, &mut ppu);

            // Write PPU
            bus.write(0x2000, 0x01);
            bus.write(0x2007, 0x02);

            assert_eq!(bus.ppu.data[0x0000], 0x01);
            assert_eq!(bus.ppu.data[0x0007], 0x02);

            // Mirror every 8 bytes
            // ref : https://www.nesdev.org/wiki/CPU_memory_map
            let mirror_count = (0x3FFF - 0x2008) / 8;
            for i in 1..=mirror_count {
                let offset = i * 8;
                let data1 = offset as u8 + 0x01;
                let data2 = offset as u8 + 0x02;

                bus.write(0x2000 + offset, data1);
                bus.write(0x2007 + offset, data2);

                assert_eq!(bus.ppu.data[0x0000], data1);
                assert_eq!(bus.ppu.data[0x0007], data2);
            }
        }
    }
}
