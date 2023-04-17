use crate::nes::{
    bus::Bus,
    cpu::{fetch, opecode::AddressingMode, registers::CpuRegisters},
};

pub fn sta<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);
    bus.write(address, registers.a);
}

pub fn stx<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);
    bus.write(address, registers.x);
}

pub fn sty<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let address = fetch::read_operand_address(bus, registers, mode);
    bus.write(address, registers.y)
}

#[cfg(test)]
mod store_tests {
    use super::*;
    use crate::nes::cpu::instructions::instructions_test::MockBus;

    #[test]
    fn sta_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        // Write absolute address 0x5001
        registers.pc = 0x0050;
        bus.write(0x0050, 0x01); // lower address
        bus.write(0x0051, 0x50); // upper address
        registers.a = 0xFF;

        sta(&mut bus, &mut registers, &AddressingMode::Absolute);

        assert_eq!(bus.read(0x5001), 0xFF);
    }

    #[test]
    fn stx_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        // Write absolute address 0x5001
        registers.pc = 0x0050;
        bus.write(0x0050, 0x01); // lower address
        bus.write(0x0051, 0x50); // upper address
        registers.x = 0xFF;

        stx(&mut bus, &mut registers, &AddressingMode::Absolute);

        assert_eq!(bus.read(0x5001), 0xFF);
    }

    #[test]
    fn sty_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        // Write absolute address 0x5001
        registers.pc = 0x0050;
        bus.write(0x0050, 0x01); // lower address
        bus.write(0x0051, 0x50); // upper address
        registers.y = 0xFF;

        sty(&mut bus, &mut registers, &AddressingMode::Absolute);

        assert_eq!(bus.read(0x5001), 0xFF);
    }
}
