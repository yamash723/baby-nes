use crate::nes::{
    bus::Bus,
    cpu::{
        fetch,
        opecode::AddressingMode,
        registers::{CpuRegisters, CpuStatusFlag},
    },
};

pub fn adc<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let a = registers.a as u16;
    let operand = fetch::fetch_operand(bus, registers, mode) as u16;
    let carry = registers.p.contains(CpuStatusFlag::CARRY) as u16;
    let computed = a.wrapping_add(operand).wrapping_add(carry);
    registers.a = computed as u8;

    registers.p.set(CpuStatusFlag::CARRY, computed > 0xFF);
    registers.p.set(CpuStatusFlag::ZERO, registers.a == 0);
    registers
        .p
        .set(CpuStatusFlag::NEGATIVE, registers.a & 0b10000000 != 0);

    // When the sign of the calculation result differs from the original sign in the addition of the same sign, it is an overflow.
    let operands_have_same_sign = ((a ^ operand) & 0b10000000) == 0;
    let result_has_different_sign = ((a ^ computed) & 0b10000000) != 0;
    registers.p.set(
        CpuStatusFlag::OVERFLOW,
        operands_have_same_sign && result_has_different_sign,
    );
}

pub fn sbc<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let a = registers.a as i16;
    let operand = fetch::fetch_operand(bus, registers, mode) as i16;
    let carry = !registers.p.contains(CpuStatusFlag::CARRY) as i16;
    let computed = a.wrapping_sub(operand).wrapping_sub(carry);
    registers.a = computed as u8;

    registers.p.set(CpuStatusFlag::CARRY, computed >= 0);
    registers.p.set(CpuStatusFlag::ZERO, registers.a == 0);
    registers
        .p
        .set(CpuStatusFlag::NEGATIVE, registers.a & 0b10000000 != 0);

    // When the sign of the calculation result differs from the original sign in the addition of the same sign, it is an overflow.
    let operands_have_same_sign = ((a ^ operand) & 0b10000000) == 0;
    let result_has_different_sign = ((a ^ computed) & 0b10000000) != 0;
    registers.p.set(
        CpuStatusFlag::OVERFLOW,
        operands_have_same_sign && result_has_different_sign,
    );
}

pub fn cmp<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let a = registers.a as i16;
    let operand = fetch::fetch_operand(bus, registers, mode) as i16;
    let computed = a.wrapping_sub(operand);

    registers.p.set(CpuStatusFlag::CARRY, computed >= 0);
    registers.p.set(CpuStatusFlag::ZERO, computed == 0);
    registers
        .p
        .set(CpuStatusFlag::NEGATIVE, computed & 0b10000000 != 0);
}

pub fn cpx<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let x = registers.x as i16;
    let operand = fetch::fetch_operand(bus, registers, mode) as i16;
    let computed = x.wrapping_sub(operand);

    registers.p.set(CpuStatusFlag::CARRY, computed >= 0);
    registers.p.set(CpuStatusFlag::ZERO, computed == 0);
    registers
        .p
        .set(CpuStatusFlag::NEGATIVE, computed & 0b10000000 != 0);
}

pub fn cpy<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let y = registers.y as i16;
    let operand = fetch::fetch_operand(bus, registers, mode) as i16;
    let computed = y.wrapping_sub(operand);

    registers.p.set(CpuStatusFlag::CARRY, computed >= 0);
    registers.p.set(CpuStatusFlag::ZERO, computed == 0);
    registers
        .p
        .set(CpuStatusFlag::NEGATIVE, computed & 0b10000000 != 0);
}

#[cfg(test)]
mod arithmetic_tests {
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
    fn adc_test() {
        struct State {
            pub a: u8,
            pub data: u8,
            pub is_set_carry_flg: bool,
            pub expect_a: u8,
            pub expect_carry_flg: bool,
            pub expect_zero_flg: bool,
            pub expect_overflow_flg: bool,
            pub expect_negative_flg: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0b00000100, data: 0b00000010, is_set_carry_flg: false, expect_a: 0b00000110, expect_carry_flg: false, expect_zero_flg: false, expect_overflow_flg: false, expect_negative_flg: false },
            State { a: 0b00000100, data: 0b00000010, is_set_carry_flg: true,  expect_a: 0b00000111, expect_carry_flg: false, expect_zero_flg: false, expect_overflow_flg: false, expect_negative_flg: false },
            State { a: 0b10000000, data: 0b10000000, is_set_carry_flg: true,  expect_a: 0b00000001, expect_carry_flg: true,  expect_zero_flg: false, expect_overflow_flg: true,  expect_negative_flg: false },
            State { a: 0b10000000, data: 0b10000000, is_set_carry_flg: false, expect_a: 0b00000000, expect_carry_flg: true,  expect_zero_flg: true,  expect_overflow_flg: true,  expect_negative_flg: false },
            State { a: 0b10000000, data: 0b00000001, is_set_carry_flg: false, expect_a: 0b10000001, expect_carry_flg: false, expect_zero_flg: false, expect_overflow_flg: false, expect_negative_flg: true  },
            State { a: 0b11000001, data: 0b01000000, is_set_carry_flg: false, expect_a: 0b00000001, expect_carry_flg: true,  expect_zero_flg: false, expect_overflow_flg: false, expect_negative_flg: false },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(registers.pc, state.data);
            registers.a = state.a;
            registers
                .p
                .set(CpuStatusFlag::CARRY, state.is_set_carry_flg);

            adc(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::CARRY),
                state.expect_carry_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::ZERO),
                state.expect_zero_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::OVERFLOW),
                state.expect_overflow_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative_flg
            );
        }
    }

    #[test]
    fn sbc_test() {
        struct State {
            pub a: u8,
            pub data: u8,
            pub is_set_carry_flg: bool,
            pub expect_a: u8,
            pub expect_carry_flg: bool,
            pub expect_zero_flg: bool,
            pub expect_overflow_flg: bool,
            pub expect_negative_flg: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0b00000110, data: 0b00000010, is_set_carry_flg: true,  expect_a: 0b00000100, expect_carry_flg: true,  expect_zero_flg: false, expect_overflow_flg: false, expect_negative_flg: false },
            State { a: 0b00000111, data: 0b00000010, is_set_carry_flg: false, expect_a: 0b00000100, expect_carry_flg: true,  expect_zero_flg: false, expect_overflow_flg: false, expect_negative_flg: false },
            State { a: 0b00000000, data: 0b01000000, is_set_carry_flg: true,  expect_a: 0b11000000, expect_carry_flg: false, expect_zero_flg: false, expect_overflow_flg: true,  expect_negative_flg: true },
            State { a: 0b00000000, data: 0b00000000, is_set_carry_flg: true,  expect_a: 0b00000000, expect_carry_flg: true,  expect_zero_flg: true,  expect_overflow_flg: false, expect_negative_flg: false },
            State { a: 0b11111111, data: 0b11111110, is_set_carry_flg: true,  expect_a: 0b00000001, expect_carry_flg: true,  expect_zero_flg: false, expect_overflow_flg: true,  expect_negative_flg: false },
            State { a: 0b11111111, data: 0b00000001, is_set_carry_flg: true,  expect_a: 0b11111110, expect_carry_flg: true,  expect_zero_flg: false, expect_overflow_flg: false, expect_negative_flg: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(registers.pc, state.data);
            registers.a = state.a;
            registers
                .p
                .set(CpuStatusFlag::CARRY, state.is_set_carry_flg);

            sbc(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::CARRY),
                state.expect_carry_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::ZERO),
                state.expect_zero_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::OVERFLOW),
                state.expect_overflow_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative_flg
            );
        }
    }

    #[test]
    fn cmp_test() {
        struct State {
            pub a: u8,
            pub data: u8,
            pub expect_carry_flg: bool,
            pub expect_zero_flg: bool,
            pub expect_negative_flg: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0x10, data: 0x01, expect_carry_flg: true,  expect_zero_flg: false, expect_negative_flg: false },
            State { a: 0x00, data: 0x00, expect_carry_flg: true,  expect_zero_flg: true,  expect_negative_flg: false },
            State { a: 0x01, data: 0x02, expect_carry_flg: false, expect_zero_flg: false, expect_negative_flg: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(registers.pc, state.data);
            registers.a = state.a;

            cmp(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(
                registers.p.contains(CpuStatusFlag::CARRY),
                state.expect_carry_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::ZERO),
                state.expect_zero_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative_flg
            );
        }
    }

    #[test]
    fn cpx_test() {
        struct State {
            pub x: u8,
            pub data: u8,
            pub expect_carry_flg: bool,
            pub expect_zero_flg: bool,
            pub expect_negative_flg: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { x: 0x10, data: 0x01, expect_carry_flg: true,  expect_zero_flg: false, expect_negative_flg: false },
            State { x: 0x00, data: 0x00, expect_carry_flg: true,  expect_zero_flg: true,  expect_negative_flg: false },
            State { x: 0x01, data: 0x02, expect_carry_flg: false, expect_zero_flg: false, expect_negative_flg: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(registers.pc, state.data);
            registers.x = state.x;

            cpx(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(
                registers.p.contains(CpuStatusFlag::CARRY),
                state.expect_carry_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::ZERO),
                state.expect_zero_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative_flg
            );
        }
    }

    #[test]
    fn cpy_test() {
        struct State {
            pub y: u8,
            pub data: u8,
            pub expect_carry_flg: bool,
            pub expect_zero_flg: bool,
            pub expect_negative_flg: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { y: 0x10, data: 0x01, expect_carry_flg: true,  expect_zero_flg: false, expect_negative_flg: false },
            State { y: 0x00, data: 0x00, expect_carry_flg: true,  expect_zero_flg: true,  expect_negative_flg: false },
            State { y: 0x01, data: 0x02, expect_carry_flg: false, expect_zero_flg: false, expect_negative_flg: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(registers.pc, state.data);
            registers.y = state.y;

            cpy(&mut bus, &mut registers, &AddressingMode::Immediate);

            assert_eq!(
                registers.p.contains(CpuStatusFlag::CARRY),
                state.expect_carry_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::ZERO),
                state.expect_zero_flg
            );
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative_flg
            );
        }
    }
}
