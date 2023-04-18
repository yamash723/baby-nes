use crate::nes::{
    bus::Bus,
    cpu::{fetch, opecode::AddressingMode, registers::CpuRegisters},
};

pub fn dex(registers: &mut CpuRegisters) {
    registers.x = registers.x.wrapping_sub(1);
    registers.update_zero_and_negative_flags(registers.x);
}

pub fn dey(registers: &mut CpuRegisters) {
    registers.y = registers.y.wrapping_sub(1);
    registers.update_zero_and_negative_flags(registers.y);
}

pub fn dec<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);
    let data = bus.read(address).wrapping_sub(1);
    registers.update_zero_and_negative_flags(data);
    bus.write(address, data)
}

#[cfg(test)]
mod decrement_tests {
    use super::*;
    use crate::nes::cpu::{
        instructions::instructions_test::MockBus, opecode::AddressingMode, registers::CpuStatusFlag,
    };

    #[test]
    fn dex_test() {
        struct State {
            pub x: u8,
            pub expect_x: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { x: 0x05, expect_x: 0x04, expect_zero: false, expect_negative: false },
            State { x: 0x01, expect_x: 0x00, expect_zero: true,  expect_negative: false },
            State { x: 0x00, expect_x: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.x = state.x;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            dex(&mut registers);

            assert_eq!(registers.x, state.expect_x);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn dey_test() {
        struct State {
            pub y: u8,
            pub expect_y: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { y: 0x05, expect_y: 0x04, expect_zero: false, expect_negative: false },
            State { y: 0x01, expect_y: 0x00, expect_zero: true,  expect_negative: false },
            State { y: 0x00, expect_y: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.y = state.y;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            dey(&mut registers);

            assert_eq!(registers.y, state.expect_y);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn dec_test() {
        struct State {
            pub address: u8,
            pub data: u8,
            pub expect_data: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { address: 0x10, data: 0x06, expect_data: 0x05, expect_zero: false, expect_negative: false },
            State { address: 0x10, data: 0x01, expect_data: 0x00, expect_zero: true,  expect_negative: false },
            State { address: 0x10, data: 0x00, expect_data: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            bus.write(registers.pc, state.address);
            bus.write(state.address as u16, state.data);

            dec(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(bus.read(state.address as u16), state.expect_data);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }
}
