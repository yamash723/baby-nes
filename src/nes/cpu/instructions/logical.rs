use crate::nes::{
    bus::Bus,
    cpu::{fetch, opecode::AddressingMode, registers::{CpuRegisters, CpuStatusFlag}},
};

pub fn and<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let operand = fetch::fetch_operand(bus, registers, mode);
    registers.a &= operand;
    registers.update_zero_and_negative_flags(registers.a);
}

pub fn eor<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let operand = fetch::fetch_operand(bus, registers, mode);
    registers.a ^= operand;
    registers.update_zero_and_negative_flags(registers.a);
}

pub fn ora<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let operand = fetch::fetch_operand(bus, registers, mode);
    registers.a |= operand;
    registers.update_zero_and_negative_flags(registers.a);
}

pub fn bit<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let operand = fetch::fetch_operand(bus, registers, mode);
    registers.p.set(CpuStatusFlag::ZERO, registers.a & operand == 0);
    registers.p.set(CpuStatusFlag::OVERFLOW, operand & 0b01000000 != 0);
    registers.p.set(CpuStatusFlag::NEGATIVE, operand & 0b10000000 != 0);
}

#[cfg(test)]
mod logical_tests {
    use super::*;
    use crate::nes::cpu::{instructions::instructions_test::MockBus, registers::CpuStatusFlag};

    #[test]
    fn and_test() {
        struct State {
            pub a: u8,
            pub data: u8,
            pub expect_a: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0b11111000, data: 0b00011111, expect_a: 0b00011000, expect_zero: false, expect_negative: false },
            State { a: 0x10, data: 0x00, expect_a: 0x00, expect_zero: true,  expect_negative: false },
            State { a: 0xFF, data: 0xFF, expect_a: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();
            registers.a = state.a;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            bus.write(registers.pc, state.data);

            and(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn eor_test() {
        struct State {
            pub a: u8,
            pub data: u8,
            pub expect_a: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0b00001110, data: 0b00000111, expect_a: 0b00001001, expect_zero: false, expect_negative: false },
            State { a: 0x00, data: 0x00, expect_a: 0x00, expect_zero: true,  expect_negative: false },
            State { a: 0x00, data: 0xFF, expect_a: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();
            registers.a = state.a;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            bus.write(registers.pc, state.data);

            eor(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn ora_test() {
        struct State {
            pub a: u8,
            pub data: u8,
            pub expect_a: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0b00001010, data: 0b00000101, expect_a: 0b00001111, expect_zero: false, expect_negative: false },
            State { a: 0x00, data: 0x00, expect_a: 0x00, expect_zero: true,  expect_negative: false },
            State { a: 0x00, data: 0xFF, expect_a: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();
            registers.a = state.a;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            bus.write(registers.pc, state.data);

            ora(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn bit_test() {
        struct State {
            pub a: u8,
            pub data: u8,
            pub expect_zero: bool,
            pub expect_overflow: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0b00000000, data: 0b00000000, expect_zero: true,  expect_overflow: false, expect_negative: false },
            State { a: 0b01000000, data: 0b01000000, expect_zero: false, expect_overflow: true,  expect_negative: false },
            State { a: 0b10000000, data: 0b10000000, expect_zero: false, expect_overflow: false, expect_negative: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();
            registers.a = state.a;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            bus.write(registers.pc, state.data);

            bit(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(registers.p.contains(CpuStatusFlag::OVERFLOW), state.expect_overflow);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }
}
