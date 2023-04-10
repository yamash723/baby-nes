use crate::nes::cpu::registers::CpuRegisters;

pub fn dey(registers: &mut CpuRegisters) {
    registers.y = registers.y.wrapping_sub(1);
    registers.update_zero_and_negative_flags(registers.y);
}

#[cfg(test)]
mod decrement_tests {
    use super::*;
    use crate::nes::cpu::registers::CpuStatusFlag;

    #[test]
    fn dey_test() {
        struct State {
            pub y: u8,
            pub expect_y: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        let patterns = vec![
            State {
                y: 0x05,
                expect_y: 0x04,
                expect_zero: false,
                expect_negative: false,
            },
            State {
                y: 0x01,
                expect_y: 0x00,
                expect_zero: true,
                expect_negative: false,
            },
            State {
                y: 0x00,
                expect_y: 0xFF,
                expect_zero: false,
                expect_negative: true,
            },
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
}
