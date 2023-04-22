use crate::nes::{
    bus::Bus,
    cpu::{fetch, opecode::AddressingMode, registers::CpuRegisters},
};

pub fn jsr<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    let operand = fetch::read_operand_address(bus, registers, mode);
    registers.push_u16(bus, registers.pc - 1);
    registers.pc = operand;
}

pub fn jmp<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    registers.pc = fetch::read_operand_address(bus, registers, mode);
}

pub fn rts<T>(bus: &mut T, registers: &mut CpuRegisters)
where
    T: Bus,
{
    registers.pc = registers.pull_u16(bus);
}

// registers.a = fetch::fetch_operand(bus, registers, mode);

#[cfg(test)]
mod jump_tests {
    use super::*;
    use crate::nes::cpu::instructions::instructions_test::MockBus;

    #[test]
    fn jmp_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        // Write absolute address 0x5001
        registers.pc = 0x0050;
        bus.write(0x0050, 0x01); // lower address
        bus.write(0x0051, 0x50); // upper address

        jmp(&mut bus, &mut registers, &AddressingMode::Absolute);

        assert_eq!(registers.pc, 0x5001);
    }

    #[test]
    fn jsr_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        // Write absolute address 0x5001
        registers.pc = 0x0103;
        bus.write(0x0103, 0x01); // lower address
        bus.write(0x0104, 0x50); // upper address

        registers.s = 0x09;
        jsr(&mut bus, &mut registers, &AddressingMode::Absolute);

        // pushed increment program counter
        assert_eq!(bus.read(0x0109), 0x01); // upper
        assert_eq!(bus.read(0x0108), 0x04); // lower

        // set operand to program counter
        assert_eq!(registers.pc, 0x5001);
    }

    #[test]
    fn rts_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        registers.s = 0x07;
        bus.write(0x0108, 0x01); // lower address
        bus.write(0x0109, 0x50); // upper address

        rts(&mut bus, &mut registers);

        // set operand to program counter
        assert_eq!(registers.pc, 0x5001);
        assert_eq!(registers.s, 0x09);
    }
}
