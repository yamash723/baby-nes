use crate::nes::cpu::registers::CpuRegisters;

pub fn inx(registers: &mut CpuRegisters) {
    registers.x = registers.x.wrapping_add(1);
    registers.update_zero_and_negative_flags(registers.x);
}

#[cfg(test)]
mod increment_tests {
    use super::*;
    use crate::nes::cpu::registers::CpuStatusFlag;

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
}
