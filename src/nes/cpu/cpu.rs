use crate::nes::bus::Bus;

use super::{
    opecode::{self, AddressingMode, Code},
    registers::{CpuRegisters, CpuStatusFlag},
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
        let mut cpu = Cpu::new(cpu_register, cpu_bus);

        let instruction_code = &cpu.fetch();
        let opecode = opecode::OPECODE_MAP.get(&instruction_code).unwrap();

        println!("{:?} : {:?} : {:?}", &opecode.code, &opecode.mode, &opecode.cycle);

        match opecode.code {
            Code::LDA => &cpu.lda(&opecode.mode),
            Code::LDX => &cpu.ldx(&opecode.mode),
            Code::LDY => &cpu.ldy(&opecode.mode),
            Code::BNE => &cpu.bne(&opecode.mode),
            Code::DEY => &cpu.dey(),
            Code::INX => &cpu.inx(),
            Code::JMP => &cpu.jmp(&opecode.mode),
            Code::SEI => &cpu.sei(),
            Code::STA => &cpu.sta(&opecode.mode),
            Code::TXS => &cpu.txs(),
        };

        opecode.cycle
    }

    fn fetch(&mut self) -> u8 {
        let data = self.bus.read(self.registers.pc);
        self.registers.pc += 1;
        data
    }

    fn fetch_relative(&mut self) -> u16 {
        let offset = self.fetch() as u16;

        if offset < 0x80 {
            self.registers.pc + offset
        } else {
            self.registers.pc + offset - 0x100
        }
    }

    fn fetch_absolute(&mut self) -> u16 {
        let lower = self.fetch() as u16;
        let upper = self.fetch() as u16;
        lower | upper << 8
    }

    fn fetch_absolute_x(&mut self) -> u16 {
        self.fetch_absolute() + self.registers.x as u16
    }

    fn fetch_opecode(&mut self, mode: &AddressingMode) -> u8 {
        if mode == &AddressingMode::Immediate {
            return self.fetch();
        }

        let address = self.read_operand_address(mode);
        self.bus.read(address)
    }

    fn read_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Absolute => self.fetch_absolute(),
            AddressingMode::AbsoluteIndexedX => self.fetch_absolute_x(),
            AddressingMode::Relative => self.fetch_relative(),
            _ => todo!(),
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        self.registers.a = self.fetch_opecode(mode);
        self.registers
            .update_zero_and_negative_flags(self.registers.a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        self.registers.x = self.fetch_opecode(mode);
        self.registers
            .update_zero_and_negative_flags(self.registers.x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        self.registers.y = self.fetch_opecode(mode);
        self.registers
            .update_zero_and_negative_flags(self.registers.y);
    }

    fn bne(&mut self, mode: &AddressingMode) {
        let address = self.read_operand_address(mode);

        if !self.registers.p.contains(CpuStatusFlag::ZERO) {
            self.registers.pc = address;
        }
    }

    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.registers
            .update_zero_and_negative_flags(self.registers.y);
    }

    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.registers
            .update_zero_and_negative_flags(self.registers.x);
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        self.registers.pc = self.read_operand_address(mode);
    }

    fn sei(&mut self) {
        self.registers.p.insert(CpuStatusFlag::INTERRUPT_DISABLE);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let address = self.read_operand_address(mode);
        self.bus.write(address, self.registers.a);
    }

    fn txs(&mut self) {
        self.registers.s = self.registers.a;
    }
}

#[cfg(test)]
mod cpu_tests {
    use crate::nes::{bus::Bus, cpu::registers::CpuRegisters};

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

        let mut cpu = Cpu::new(&mut registers, &mut bus);
        let operand = &cpu.fetch();

        assert_eq!(*operand, 10);
        assert_eq!(cpu.registers.pc, 3);
    }
}
