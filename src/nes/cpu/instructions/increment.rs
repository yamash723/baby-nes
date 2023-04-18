use crate::nes::{
    bus::Bus,
    cpu::{fetch, opecode::AddressingMode, registers::CpuRegisters},
};

pub fn inx(registers: &mut CpuRegisters) {
    registers.x = registers.x.wrapping_add(1);
    registers.update_zero_and_negative_flags(registers.x);
}

pub fn iny(registers: &mut CpuRegisters) {
    registers.y = registers.y.wrapping_add(1);
    registers.update_zero_and_negative_flags(registers.y);
}

pub fn inc<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);
    let data = bus.read(address).wrapping_add(1);
    registers.update_zero_and_negative_flags(data);
    bus.write(address, data)
}

#[cfg(test)]
mod increment_tests {
    use super::*;
    use crate::nes::cpu::{instructions::instructions_test::MockBus, registers::CpuStatusFlag};

    #[test]
    fn inx_test() {
        struct State {
            pub x: u8,
            pub expect_x: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { x: 0x05, expect_x: 0x06, expect_zero: false, expect_negative: false },
            State { x: 0xFF, expect_x: 0x00, expect_zero: true,  expect_negative: false },
            State { x: 0xFE, expect_x: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.x = state.x;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            inx(&mut registers);

            assert_eq!(registers.x, state.expect_x);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn iny_test() {
        struct State {
            pub y: u8,
            pub expect_y: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { y: 0x05, expect_y: 0x06, expect_zero: false, expect_negative: false },
            State { y: 0xFF, expect_y: 0x00, expect_zero: true,  expect_negative: false },
            State { y: 0xFE, expect_y: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.y = state.y;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            iny(&mut registers);

            assert_eq!(registers.y, state.expect_y);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn inc_test() {
        struct State {
            pub address: u8,
            pub data: u8,
            pub expect_data: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { address: 0x10, data: 0x05, expect_data: 0x06, expect_zero: false, expect_negative: false },
            State { address: 0x10, data: 0xFF, expect_data: 0x00, expect_zero: true,  expect_negative: false },
            State { address: 0x10, data: 0xFE, expect_data: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            bus.write(registers.pc, state.address);
            bus.write(state.address as u16, state.data);

            inc(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(bus.read(state.address as u16), state.expect_data);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }
}
