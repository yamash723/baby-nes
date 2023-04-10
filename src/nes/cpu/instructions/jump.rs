use crate::nes::{
    bus::Bus,
    cpu::{fetch, opecode::AddressingMode, registers::CpuRegisters},
};

pub fn jmp<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode)
where
    T: Bus,
{
    registers.pc = fetch::read_operand_address(bus, registers, mode);
}

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
}
