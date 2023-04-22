use bitflags::bitflags;

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
}

#[cfg(test)]
mod registers_tests {
    use super::*;

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
}
