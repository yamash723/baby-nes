use super::{registers::{Registers, CpuStatusFlag}, opecode::{self, Code, AddressingMode}};

pub struct Cpu<'a, T: CpuBus> {
    registers: Registers,
    bus: &'a mut T,
}

pub trait CpuBus {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

impl <'a, T: CpuBus> Cpu<'a, T> {
    pub fn new(registers: Registers, bus: &'a mut T) -> Self {
        Self { registers, bus }
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

    pub fn run(&mut self) {
        let instruction_code = &self.fetch();
        let opecode = opecode::OPECODE_MAP.get(&instruction_code).unwrap();

        match opecode.code {    
            Code::LDA => &self.lda(&opecode.mode),
            Code::LDX => &self.ldx(&opecode.mode),
            Code::LDY => &self.ldy(&opecode.mode),
            Code::BNE => &self.bne(&opecode.mode),
            Code::DEY => &self.dey(),
            Code::INX => &self.inx(),
            Code::JMP => &self.jmp(&opecode.mode),
            Code::SEI => &self.sei(),
            Code::STA => &self.sta(&opecode.mode),
            Code::TXS => &self.txs(),
        };
    }

    fn read_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Implied => self.registers.pc,
            AddressingMode::Accumulator => todo!(),
            AddressingMode::Immediate => todo!(),
            AddressingMode::ZeroPage => todo!(),
            AddressingMode::ZeroPageIndexedX => todo!(),
            AddressingMode::ZeroPageIndexedY => todo!(),
            AddressingMode::Absolute => todo!(),
            AddressingMode::AbsoluteIndexedX => todo!(),
            AddressingMode::AbsoluteIndexedY => todo!(),
            AddressingMode::Relative => self.fetch_relative(),
            AddressingMode::IndexedIndirect => todo!(),
            AddressingMode::IndirectIndexed => todo!(),
            AddressingMode::AbsoluteIndirect => todo!(),
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let address = self.read_operand_address(mode);
        self.registers.a = self.bus.read(address);
        self.registers.update_zero_and_negative_flags(self.registers.a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let address = self.read_operand_address(mode);
        self.registers.x = self.bus.read(address);
        self.registers.update_zero_and_negative_flags(self.registers.x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let address = self.read_operand_address(mode);
        self.registers.y = self.bus.read(address);
        self.registers.update_zero_and_negative_flags(self.registers.y);
    }

    fn bne(&mut self, mode: &AddressingMode) {
        let address = self.read_operand_address(mode);

        if self.registers.p.contains(CpuStatusFlag::ZERO) {
            self.registers.pc = address;
        }
    }

    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.registers.update_zero_and_negative_flags(self.registers.y);
    }

    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.registers.update_zero_and_negative_flags(self.registers.x);
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
    use crate::nes::cpu::{registers::Registers};
    use super::{Cpu, CpuBus};

    struct MockBus {
        data: Vec<u8>,
    }

    impl CpuBus for MockBus {
        fn read(&self, address: u16) -> u8 {
            self.data[address as usize]
        }

        fn write(&mut self, address: u16, data: u8) {
            self.data[address as usize] = data
        }
    }

    #[test]
    fn fetch_should_increment_pc_test() {
        let mut registers = Registers::new();

        let mut bus = MockBus { data: vec![1, 1, 10] };
        registers.pc = 2;

        let mut cpu = Cpu::new(registers, &mut bus);
        let operand = &cpu.fetch();

        assert_eq!(*operand, 10);
        assert_eq!(cpu.registers.pc, 3);
    }
}
