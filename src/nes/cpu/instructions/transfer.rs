use crate::nes::cpu::registers::CpuRegisters;

pub fn tax(registers: &mut CpuRegisters) {
    registers.x = registers.a;
    registers.update_zero_and_negative_flags(registers.x);
}

pub fn tay(registers: &mut CpuRegisters) {
    registers.y = registers.a;
    registers.update_zero_and_negative_flags(registers.y);
}

pub fn txa(registers: &mut CpuRegisters) {
    registers.a = registers.x;
    registers.update_zero_and_negative_flags(registers.a);
}

pub fn tya(registers: &mut CpuRegisters) {
    registers.a = registers.y;
    registers.update_zero_and_negative_flags(registers.a);
}

#[cfg(test)]
mod transfer_tests {
    use crate::nes::cpu::registers::CpuStatusFlag;

    use super::*;

    #[test]
    fn tax_test() {
        struct State {
            pub a: u8,
            pub expect_x: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0x05, expect_x: 0x05, expect_zero: false, expect_negative: false },
            State { a: 0x00, expect_x: 0x00, expect_zero: true,  expect_negative: false },
            State { a: 0xFF, expect_x: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.a = state.a;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            tax(&mut registers);

            assert_eq!(registers.x, state.expect_x);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn tay_test() {
        struct State {
            pub a: u8,
            pub expect_y: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { a: 0x05, expect_y: 0x05, expect_zero: false, expect_negative: false },
            State { a: 0x00, expect_y: 0x00, expect_zero: true,  expect_negative: false },
            State { a: 0xFF, expect_y: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.a = state.a;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            tay(&mut registers);

            assert_eq!(registers.y, state.expect_y);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn txa_test() {
        struct State {
            pub x: u8,
            pub expect_a: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { x: 0x05, expect_a: 0x05, expect_zero: false, expect_negative: false },
            State { x: 0x00, expect_a: 0x00, expect_zero: true,  expect_negative: false },
            State { x: 0xFF, expect_a: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.x = state.x;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            txa(&mut registers);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn tya_test() {
        struct State {
            pub y: u8,
            pub expect_a: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { y: 0x05, expect_a: 0x05, expect_zero: false, expect_negative: false },
            State { y: 0x00, expect_a: 0x00, expect_zero: true,  expect_negative: false },
            State { y: 0xFF, expect_a: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.y = state.y;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            tya(&mut registers);

            assert_eq!(registers.a, state.expect_a);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }
}
