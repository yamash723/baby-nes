use crate::nes::{
    bus::Bus,
    cpu::{fetch, opecode::AddressingMode, registers::CpuRegisters},
};

pub fn lda<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    registers.a = fetch::fetch_operand(bus, registers, mode);
    registers.update_zero_and_negative_flags(registers.a);
}

pub fn ldx<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    registers.x = fetch::fetch_operand(bus, registers, mode);
    registers.update_zero_and_negative_flags(registers.x);
}

pub fn ldy<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    registers.y = fetch::fetch_operand(bus, registers, mode);
    registers.update_zero_and_negative_flags(registers.y);
}

#[cfg(test)]
mod store_tests {
    use crate::nes::{
        bus::Bus,
        cpu::{
            instructions::instructions_test::MockBus,
            opecode::AddressingMode,
            registers::{CpuRegisters, CpuStatusFlag},
        },
    };

    use super::*;

    #[test]
    fn lda_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub expect_a: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { pc: 0x0005, data: 0x01, expect_a: 0x01, expect_zero: false, expect_negative: false },
            State { pc: 0x0005, data: 0x00, expect_a: 0x00, expect_zero: true,  expect_negative: false },
            State { pc: 0x0005, data: 0x80, expect_a: 0x80, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;

            lda(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn ldx_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub expect_x: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { pc: 0x0005, data: 0x01, expect_x: 0x01, expect_zero: false, expect_negative: false },
            State { pc: 0x0005, data: 0x00, expect_x: 0x00, expect_zero: true,  expect_negative: false },
            State { pc: 0x0005, data: 0x80, expect_x: 0x80, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;

            ldx(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(registers.x, state.expect_x);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn ldy_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub expect_y: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { pc: 0x0005, data: 0x01, expect_y: 0x01, expect_zero: false, expect_negative: false, },
            State { pc: 0x0005, data: 0x00, expect_y: 0x00, expect_zero: true,  expect_negative: false, },
            State { pc: 0x0005, data: 0x80, expect_y: 0x80, expect_zero: false, expect_negative: true, },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;

            ldy(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(registers.y, state.expect_y);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }
}
