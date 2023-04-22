use crate::nes::{
    bus::Bus,
    cpu::registers::{CpuRegisters, CpuStatusFlag},
};

pub fn tsx(registers: &mut CpuRegisters) {
    registers.x = registers.s;
    registers.update_zero_and_negative_flags(registers.x);
}

pub fn txs(registers: &mut CpuRegisters) {
    registers.s = registers.x;
}

pub fn pha<T>(bus: &mut T, registers: &mut CpuRegisters)
where
    T: Bus,
{
    registers.push(bus, registers.a);
}

pub fn php<T>(bus: &mut T, registers: &mut CpuRegisters)
where
    T: Bus,
{
    registers.push(bus, registers.p.bits());
}

pub fn pla<T>(bus: &mut T, registers: &mut CpuRegisters)
where
    T: Bus,
{
    registers.a = registers.pull(bus);
    registers.update_zero_and_negative_flags(registers.a);
}

pub fn plp<T>(bus: &mut T, registers: &mut CpuRegisters)
where
    T: Bus,
{
    let data = registers.pull(bus);
    registers.p = CpuStatusFlag::from_bits_truncate(data);
}

#[cfg(test)]
mod stack_tests {
    use crate::nes::cpu::{instructions::instructions_test::MockBus, registers::CpuStatusFlag};

    use super::*;

    #[test]
    fn tsx_test() {
        struct State {
            pub s: u8,
            pub expect_x: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { s: 0x05, expect_x: 0x05, expect_zero: false, expect_negative: false },
            State { s: 0x00, expect_x: 0x00, expect_zero: true,  expect_negative: false },
            State { s: 0xFF, expect_x: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut registers = CpuRegisters::new();
            registers.s = state.s;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);

            tsx(&mut registers);

            assert_eq!(registers.x, state.expect_x);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn txs_test() {
        let mut registers = CpuRegisters::new();
        registers.x = 0x10;

        txs(&mut registers);

        assert_eq!(registers.s, 0x10);
    }

    #[test]
    fn pha_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        registers.s = 0x9;
        registers.a = 0x20;

        pha(&mut bus, &mut registers);

        assert_eq!(registers.s, 0x08);
        assert_eq!(bus.read(0x0109), registers.a);
    }

    #[test]
    fn php_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        registers.s = 0x9;
        registers.p = CpuStatusFlag::from_bits_truncate(0x10);

        php(&mut bus, &mut registers);

        assert_eq!(registers.s, 0x08);
        assert_eq!(bus.read(0x0109), registers.p.bits());
    }

    #[test]
    fn pla_test() {
        struct State {
            pub s: u8,
            pub expect_s: u8,
            pub stack_data: u8,
            pub expect_a: u8,
            pub expect_zero: bool,
            pub expect_negative: bool,
        }

        #[rustfmt::skip]
        let patterns = vec![
            State { s: 0x09, expect_s: 0x0A, stack_data: 0x05, expect_a: 0x05, expect_zero: false, expect_negative: false },
            State { s: 0x09, expect_s: 0x0A, stack_data: 0x00, expect_a: 0x00, expect_zero: true,  expect_negative: false },
            State { s: 0x09, expect_s: 0x0A, stack_data: 0xFF, expect_a: 0xFF, expect_zero: false, expect_negative: true },
        ];

        for state in patterns {
            let mut bus = MockBus::new();
            let mut registers = CpuRegisters::new();
            registers.s = state.s;
            registers.p.remove(CpuStatusFlag::ZERO);
            registers.p.remove(CpuStatusFlag::NEGATIVE);
            bus.write(0x0100 | (state.s + 1) as u16, state.stack_data);

            pla(&mut bus, &mut registers);

            assert_eq!(registers.s, state.expect_s);
            assert_eq!(registers.a, state.expect_a);
            assert_eq!(registers.p.contains(CpuStatusFlag::ZERO), state.expect_zero);
            assert_eq!(
                registers.p.contains(CpuStatusFlag::NEGATIVE),
                state.expect_negative
            );
        }
    }

    #[test]
    fn plp_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        registers.s = 0x08;
        bus.write(0x0109, 0x20);

        plp(&mut bus, &mut registers);

        assert_eq!(registers.s, 0x09);
        assert_eq!(registers.p.bits(), 0x20);
    }
}
