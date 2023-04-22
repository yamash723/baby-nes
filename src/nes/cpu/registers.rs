use bitflags::bitflags;

use crate::nes::bus::Bus;

const DEFAULT_STACK_POINT: u8 = 0xFD;

bitflags! {
    pub struct CpuStatusFlag: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL           = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

pub struct CpuRegisters {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
    pub p: CpuStatusFlag,
    pub pc: u16,
}

impl CpuRegisters {
    pub fn new() -> Self {
        CpuRegisters {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            s: DEFAULT_STACK_POINT,
            p: CpuStatusFlag { bits: 0x00000000 },
            pc: 0x8000,
        }
    }

    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.p.insert(CpuStatusFlag::ZERO);
        } else {
            self.p.remove(CpuStatusFlag::ZERO);
        }

        if result >> 7 == 1 {
            self.p.insert(CpuStatusFlag::NEGATIVE);
        } else {
            self.p.remove(CpuStatusFlag::NEGATIVE);
        }
    }

    pub fn push<T>(&mut self, bus: &mut T, data: u8)
    where
        T: Bus,
    {
        bus.write(0x0100 | self.s as u16, data);
        self.s = self.s.wrapping_sub(1);
    }

    pub fn pull<T>(&mut self, bus: &mut T) -> u8
    where
        T: Bus,
    {
        self.s = self.s.wrapping_add(1);
        bus.read(0x0100 | self.s as u16)
    }
}

#[cfg(test)]
mod registers_tests {
    use super::*;

    pub struct MockBus {
        data: Vec<u8>,
    }

    impl MockBus {
        pub fn new() -> Self {
            Self {
                data: vec![0; 0xFFFF],
            }
        }
    }

    impl Bus for MockBus {
        fn read(&self, address: u16) -> u8 {
            self.data[address as usize]
        }

        fn read_u16(&self, _address: u16) -> u16 {
            unimplemented!()
        }

        fn write(&mut self, address: u16, data: u8) {
            self.data[address as usize] = data;
        }
    }

    #[test]
    fn update_zero_and_negative_flags_test() {
        struct State {
            pub data: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { data: 0x10, expect_zero: false, expect_negative: false },
            State { data: 0x00, expect_zero: true,  expect_negative: false },
            State { data: 0x80, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.p.set(CpuStatusFlag::ZERO, !state.expect_zero);
            registers
                .p
                .set(CpuStatusFlag::NEGATIVE, !state.expect_negative);

            registers.update_zero_and_negative_flags(state.data);

            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn push_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        registers.s = 0x09;
        registers.push(&mut bus, 0x20);

        assert_eq!(registers.s, 0x08);
        assert_eq!(bus.read(0x0109), 0x20)
    }

    #[test]
    fn pull_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        registers.s = 0x09;
        bus.write(0x010A, 0x20);

        let data = registers.pull(&mut bus);

        assert_eq!(registers.s, 0x0A);
        assert_eq!(data, 0x20);
    }
}
