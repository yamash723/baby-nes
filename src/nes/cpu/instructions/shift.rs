use crate::nes::cpu::registers::{CpuRegisters, CpuStatusFlag};

pub fn asl(registers: &mut CpuRegisters) {
    let is_carry = registers.a & 0b10000000 != 0;
    registers.a <<= 1;

    registers.p.set(CpuStatusFlag::CARRY, is_carry);
    registers.update_zero_and_negative_flags(registers.a);
}

pub fn lsr(registers: &mut CpuRegisters) {
    let is_carry = registers.a & 0b00000001 != 0;
    registers.a >>= 1;

    registers.p.set(CpuStatusFlag::CARRY, is_carry);
    registers.update_zero_and_negative_flags(registers.a);
}

pub fn rol(registers: &mut CpuRegisters) {
    let is_carry = registers.a & 0b10000000 != 0;
    registers.a <<= 1;

    if registers.p.contains(CpuStatusFlag::CARRY) {
        registers.a |= 0b00000001;
    }

    registers.p.set(CpuStatusFlag::CARRY, is_carry);
    registers.update_zero_and_negative_flags(registers.a);
}

pub fn ror(registers: &mut CpuRegisters) {
    let is_carry = registers.a & 0b00000001 != 0;
    registers.a >>= 1;

    if registers.p.contains(CpuStatusFlag::CARRY) {
        registers.a |= 0b10000000;
    }

    registers.p.set(CpuStatusFlag::CARRY, is_carry);
    registers.update_zero_and_negative_flags(registers.a);
}

#[cfg(test)]
mod shift_tests {
    use super::*;
    use crate::nes::cpu::registers::CpuStatusFlag;

    #[test]
    fn asl_test() {
        struct State {
            pub a: u8,
            pub expect_a: u8,
            pub expect_carry: bool,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0b00000011, expect_a: 0b00000110, expect_carry: false, expect_zero: false, expect_negative: false },
            State { a: 0b10000001, expect_a: 0b00000010, expect_carry: true,  expect_zero: false, expect_negative: false },
            State { a: 0b10000000, expect_a: 0b00000000, expect_carry: true,  expect_zero: true,  expect_negative: false },
            State { a: 0b01111111, expect_a: 0b11111110, expect_carry: false, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.a = state.a;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            asl(&mut registers);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::CARRY),
                state.expect_carry
            );
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn lsr_test() {
        struct State {
            pub a: u8,
            pub expect_a: u8,
            pub expect_carry: bool,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0b00000110, expect_a: 0b00000011, expect_carry: false, expect_zero: false, expect_negative: false },
            State { a: 0b00000011, expect_a: 0b00000001, expect_carry: true,  expect_zero: false, expect_negative: false },
            State { a: 0b00000001, expect_a: 0b00000000, expect_carry: true,  expect_zero: true,  expect_negative: false },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.a = state.a;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            lsr(&mut registers);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::CARRY),
                state.expect_carry
            );
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn rol_test() {
        struct State {
            pub a: u8,
            pub carry: bool,
            pub expect_a: u8,
            pub expect_carry: bool,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0b00000011, carry: false, expect_a: 0b00000110, expect_carry: false, expect_zero: false, expect_negative: false },
            State { a: 0b00000011, carry: true, expect_a: 0b00000111, expect_carry: false, expect_zero: false, expect_negative: false },
            State { a: 0b10000001, carry: false, expect_a: 0b00000010, expect_carry: true,  expect_zero: false, expect_negative: false },
            State { a: 0b10000000, carry: false, expect_a: 0b00000000, expect_carry: true,  expect_zero: true,  expect_negative: false },
            State { a: 0b01111111, carry: false, expect_a: 0b11111110, expect_carry: false, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.a = state.a;
            registers.p.set(CpuStatusFlag::CARRY, state.carry);
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            rol(&mut registers);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::CARRY),
                state.expect_carry
            );
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn ror_test() {
        struct State {
            pub a: u8,
            pub carry: bool,
            pub expect_a: u8,
            pub expect_carry: bool,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0b00000110, carry: false, expect_a: 0b00000011, expect_carry: false, expect_zero: false, expect_negative: false },
            State { a: 0b00000110, carry: true,  expect_a: 0b10000011, expect_carry: false, expect_zero: false, expect_negative: true },
            State { a: 0b10000001, carry: false, expect_a: 0b01000000, expect_carry: true,  expect_zero: false, expect_negative: false },
            State { a: 0b00000001, carry: false, expect_a: 0b00000000, expect_carry: true,  expect_zero: true,  expect_negative: false },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.a = state.a;
            registers.p.set(CpuStatusFlag::CARRY, state.carry);
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            ror(&mut registers);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::CARRY),
                state.expect_carry
            );
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }
}
