use crate::nes::{
    bus::Bus,
    cpu::registers::{CpuRegisters, CpuStatusFlag},
};

pub fn brk<T>(bus: &mut T, registers: &mut CpuRegisters)
where
    T: Bus,
{
    registers.push_u16(bus, registers.pc);
    registers.push(bus, registers.p.bits());

    registers.pc = bus.read_u16(0xFFFE);

    registers.p.insert(CpuStatusFlag::BREAK);
    registers.p.insert(CpuStatusFlag::INTERRUPT_DISABLE);
}

pub fn rti<T>(bus: &mut T, registers: &mut CpuRegisters)
where
    T: Bus,
{
    let data = registers.pull(bus);
    registers.p = CpuStatusFlag::from_bits_truncate(data);

    let lower = registers.pull(bus);
    let upper = registers.pull(bus);
    registers.pc = u16::from_be_bytes([upper, lower]);
}

#[cfg(test)]
mod system_tests {
    use crate::nes::cpu::{instructions::instructions_test::MockBus};

    use super::*;

    #[test]
    fn brk_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        registers.s = 0x09;
        registers.pc = 0xFF20;
        registers.p = CpuStatusFlag::from_bits_truncate(0b10010101);
        bus.write(0xFFFE, 0x12);

        brk(&mut bus, &mut registers);

        assert_eq!(bus.read(0x0107), 0b10010101);
        assert_eq!(bus.read(0x0108), 0x20);
        assert_eq!(bus.read(0x0109), 0xFF);
        assert_eq!(registers.s, 0x06);
        assert_eq!(registers.pc, 0x12);
        assert!(registers.p.contains(CpuStatusFlag::BREAK));
        assert!(registers.p.contains(CpuStatusFlag::INTERRUPT_DISABLE));
    }

    #[test]
    fn rti_test() {
        let mut bus = MockBus::new();
        let mut registers = CpuRegisters::new();

        registers.s = 0x06;
        bus.write(0x0107, 0x20);
        bus.write(0x0108, 0x20);
        bus.write(0x0109, 0xFF);

        rti(&mut bus, &mut registers);

        assert_eq!(registers.s, 0x09);
        assert_eq!(registers.p.bits(), 0x20);
        assert_eq!(registers.pc, 0xFF20);
    }
}
