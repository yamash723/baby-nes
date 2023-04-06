use crate::nes::{bus::Bus, cpu::instructions};

use super::{
    opecode::{self, Code},
    registers::{CpuRegisters}, fetch,
};

pub struct Cpu<'a, T: Bus> {
    registers: &'a mut CpuRegisters,
    bus: &'a mut T,
}

impl<'a, T: Bus> Cpu<'a, T> {
    fn new(registers: &'a mut CpuRegisters, bus: &'a mut T) -> Self {
        Self { registers, bus }
    }

    pub fn run(cpu_register: &'a mut CpuRegisters, cpu_bus: &mut T) -> u16
    where
        T: Bus,
    {
        let cpu = Cpu::new(cpu_register, cpu_bus);

        let instruction_code = fetch::fetch(cpu.bus, cpu.registers);
        let opecode = opecode::OPECODE_MAP.get(&instruction_code).unwrap();

        println!("{:?} : {:?} : {:?}", &opecode.code, &opecode.mode, &opecode.cycle);

        match opecode.code {
            // ref: https://www.nesdev.org/obelisk-6502-guide/instructions.html
            Code::LDA => instructions::load::lda(cpu.bus, cpu.registers, &opecode.mode),
            Code::LDX => instructions::load::ldx(cpu.bus, cpu.registers, &opecode.mode),
            Code::LDY => instructions::load::ldy(cpu.bus, cpu.registers, &opecode.mode),
            Code::STA => instructions::store::sta(cpu.bus, cpu.registers, &opecode.mode),
            Code::BNE => instructions::branch::bne(cpu.bus, cpu.registers, &opecode.mode),
            Code::DEY => instructions::decrement::dey(cpu.registers),
            Code::INX => instructions::increment::inx(cpu.registers),
            Code::JMP => instructions::jump::jmp(cpu.bus, cpu.registers, &opecode.mode),
            Code::SEI => instructions::flags::sei(cpu.registers),
            Code::TXS => instructions::transfer::txs(cpu.registers),
        };

        opecode.cycle
    }
}

#[cfg(test)]
mod cpu_tests {
    use crate::nes::{bus::Bus, cpu::{registers::CpuRegisters, fetch}};

    use super::Cpu;

    struct MockBus {
        data: Vec<u8>,
    }

    impl Bus for MockBus {
        fn read(&self, address: u16) -> u8 {
            self.data[address as usize]
        }

        fn write(&mut self, address: u16, data: u8) {
            self.data[address as usize] = data
        }
    }

    #[test]
    fn fetch_should_increment_pc_test() {
        let mut registers = CpuRegisters::new();

        let mut bus = MockBus {
            data: vec![1, 1, 10],
        };
        registers.pc = 2;

        let cpu = Cpu::new(&mut registers, &mut bus);
        let operand = fetch::fetch(cpu.bus, cpu.registers);

        assert_eq!(operand, 10);
        assert_eq!(cpu.registers.pc, 3);
    }
}
