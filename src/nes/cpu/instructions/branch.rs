use crate::nes::{
    bus::Bus,
    cpu::{
        fetch,
        opecode::AddressingMode,
        registers::{CpuRegisters, CpuStatusFlag},
    },
};


pub fn bcc<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);

    if !registers.p.contains(CpuStatusFlag::CARRY) {
        registers.pc = address;
    }
}

pub fn bcs<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);

    if registers.p.contains(CpuStatusFlag::CARRY) {
        registers.pc = address;
    }
}

pub fn beq<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);

    if registers.p.contains(CpuStatusFlag::ZERO) {
        registers.pc = address;
    }
}

pub fn bmi<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);

    if registers.p.contains(CpuStatusFlag::NEGATIVE) {
        registers.pc = address;
    }
}

pub fn bne<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);

    if !registers.p.contains(CpuStatusFlag::ZERO) {
        registers.pc = address;
    }
}

pub fn bpl<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);

    if !registers.p.contains(CpuStatusFlag::NEGATIVE) {
        registers.pc = address;
    }
}

pub fn bvc<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);

    if !registers.p.contains(CpuStatusFlag::OVERFLOW) {
        registers.pc = address;
    }
}

pub fn bvs<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);

    if registers.p.contains(CpuStatusFlag::OVERFLOW) {
        registers.pc = address;
    }
}

#[cfg(test)]
mod branch_tests {
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
    fn bcc_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub is_set_carry_flg: bool,
            pub expect_pc: u16,
        }

        #[rustfmt::skip]
        let patterns = vec![
            // BCC should be executed when carry flag is not set. (increment pc & add data)
            State { pc: 0x0005, data: 0x50, is_set_carry_flg: false, expect_pc: 0x0005 + 1 + 0x0050 },
            // BCC should not be executed when carry flag is set. (increment pc only)
            State { pc: 0x0005, data: 0x50, is_set_carry_flg: true,  expect_pc: 0x0005 + 1 },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;
            registers.p.set(CpuStatusFlag::CARRY, state.is_set_carry_flg);

            bcc(&mut bus, &mut registers, &AddressingMode::Relative);

            assert_eq!(registers.pc, state.expect_pc);
        }
    }

    #[test]
    fn bcs_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub is_set_carry_flg: bool,
            pub expect_pc: u16,
        }

        #[rustfmt::skip]
        let patterns = vec![
            // BCS should be executed when carry flag is set. (increment pc & add data)
            State { pc: 0x0005, data: 0x50, is_set_carry_flg: true,  expect_pc: 0x0005 + 1 + 0x0050 },
            // BCS should not be executed when carry flag is not set. (increment pc only)
            State { pc: 0x0005, data: 0x50, is_set_carry_flg: false, expect_pc: 0x0005 + 1 },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;
            registers.p.set(CpuStatusFlag::CARRY, state.is_set_carry_flg);

            bcs(&mut bus, &mut registers, &AddressingMode::Relative);

            assert_eq!(registers.pc, state.expect_pc);
        }
    }

    #[test]
    fn beq_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub is_set_zero_flg: bool,
            pub expect_pc: u16,
        }

        #[rustfmt::skip]
        let patterns = vec![
            // BEQ should be executed when zero flag is set. (increment pc & add data)
            State { pc: 0x0005, data: 0x50, is_set_zero_flg: true,  expect_pc: 0x0005 + 1 + 0x0050 },
            // BEQ should not be executed when zero flag is not set. (increment pc only)
            State { pc: 0x0005, data: 0x50, is_set_zero_flg: false, expect_pc: 0x0005 + 1 },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;
            registers.p.set(CpuStatusFlag::ZERO, state.is_set_zero_flg);

            beq(&mut bus, &mut registers, &AddressingMode::Relative);

            assert_eq!(registers.pc, state.expect_pc);
        }
    }

    #[test]
    fn bmi_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub is_set_negative_flg: bool,
            pub expect_pc: u16,
        }

        #[rustfmt::skip]
        let patterns = vec![
            // BMI should be executed when negative flag is set. (increment pc & add data)
            State { pc: 0x0005, data: 0x50, is_set_negative_flg: true,  expect_pc: 0x0005 + 1 + 0x0050 },
            // BMI should not be executed when negative flag is not set. (increment pc only)
            State { pc: 0x0005, data: 0x50, is_set_negative_flg: false, expect_pc: 0x0005 + 1 },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;
            registers.p.set(CpuStatusFlag::NEGATIVE, state.is_set_negative_flg);

            bmi(&mut bus, &mut registers, &AddressingMode::Relative);

            assert_eq!(registers.pc, state.expect_pc);
        }
    }

    #[test]
    fn bne_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub is_set_zero_flg: bool,
            pub expect_pc: u16,
        }

        #[rustfmt::skip]
        let patterns = vec![
            // BNE should be executed when zero flag is not set. (increment pc & add data)
            State { pc: 0x0005, data: 0x50, is_set_zero_flg: false, expect_pc: 0x0005 + 1 + 0x0050 },
            // BNE should not be executed when zero flag is set. (increment pc only)
            State { pc: 0x0005, data: 0x50, is_set_zero_flg: true,  expect_pc: 0x0005 + 1 },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;
            registers.p.set(CpuStatusFlag::ZERO, state.is_set_zero_flg);

            bne(&mut bus, &mut registers, &AddressingMode::Relative);

            assert_eq!(registers.pc, state.expect_pc);
        }
    }

    #[test]
    fn bpl_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub is_set_negative_flg: bool,
            pub expect_pc: u16,
        }

        #[rustfmt::skip]
        let patterns = vec![
            // BPL should be executed when negative flag is not set. (increment pc & add data)
            State { pc: 0x0005, data: 0x50, is_set_negative_flg: false, expect_pc: 0x0005 + 1 + 0x0050 },
            // BPL should not be executed when negative flag is set. (increment pc only)
            State { pc: 0x0005, data: 0x50, is_set_negative_flg: true,  expect_pc: 0x0005 + 1 },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;
            registers.p.set(CpuStatusFlag::NEGATIVE, state.is_set_negative_flg);

            bpl(&mut bus, &mut registers, &AddressingMode::Relative);

            assert_eq!(registers.pc, state.expect_pc);
        }
    }

    #[test]
    fn bvc_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub is_set_overflow_flg: bool,
            pub expect_pc: u16,
        }

        #[rustfmt::skip]
        let patterns = vec![
            // BVC should be executed when overflow flag is not set. (increment pc & add data)
            State { pc: 0x0005, data: 0x50, is_set_overflow_flg: false, expect_pc: 0x0005 + 1 + 0x0050 },
            // BVC should not be executed when overflow flag is set. (increment pc only)
            State { pc: 0x0005, data: 0x50, is_set_overflow_flg: true,  expect_pc: 0x0005 + 1 },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;
            registers.p.set(CpuStatusFlag::OVERFLOW, state.is_set_overflow_flg);

            bvc(&mut bus, &mut registers, &AddressingMode::Relative);

            assert_eq!(registers.pc, state.expect_pc);
        }
    }

    #[test]
    fn bvs_test() {
        struct State {
            pub pc: u16,
            pub data: u8,
            pub is_set_overflow_flg: bool,
            pub expect_pc: u16,
        }

        #[rustfmt::skip]
        let patterns = vec![
            // BVS should be executed when overflow flag is set. (increment pc & add data)
            State { pc: 0x0005, data: 0x50, is_set_overflow_flg: true,  expect_pc: 0x0005 + 1 + 0x0050 },
            // BVS should not be executed when overflow flag is not set. (increment pc only)
            State { pc: 0x0005, data: 0x50, is_set_overflow_flg: false, expect_pc: 0x0005 + 1 },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();

            bus.write(state.pc, state.data);
            registers.pc = state.pc;
            registers.p.set(CpuStatusFlag::OVERFLOW, state.is_set_overflow_flg);

            bvs(&mut bus, &mut registers, &AddressingMode::Relative);

            assert_eq!(registers.pc, state.expect_pc);
        }
    }
}
