use crate::nes::{bus::Bus, cpu::instructions};

use super::{
    fetch,
    opecode::{self, Code},
    registers::CpuRegisters,
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

        println!(
            "{:?} : {:?} : {:?}",
            &opecode.code, &opecode.mode, &opecode.cycle
        );

        match opecode.code {
            // ref: https://www.nesdev.org/obelisk-6502-guide/instructions.html
            // -- Load --
            Code::LDA => instructions::load::lda(cpu.bus, cpu.registers, &opecode.mode),
            Code::LDX => instructions::load::ldx(cpu.bus, cpu.registers, &opecode.mode),
            Code::LDY => instructions::load::ldy(cpu.bus, cpu.registers, &opecode.mode),
            // -- Store --
            Code::STA => instructions::store::sta(cpu.bus, cpu.registers, &opecode.mode),
            Code::STX => instructions::store::stx(cpu.bus, cpu.registers, &opecode.mode),
            Code::STY => instructions::store::sty(cpu.bus, cpu.registers, &opecode.mode),
            // -- Transfer --
            Code::TAX => instructions::transfer::tax(cpu.registers),
            Code::TAY => instructions::transfer::tay(cpu.registers),
            Code::TXA => instructions::transfer::txa(cpu.registers),
            Code::TYA => instructions::transfer::tya(cpu.registers),
            // -- Stack --
            Code::TSX => instructions::stack::tsx(cpu.registers),
            Code::TXS => instructions::stack::txs(cpu.registers),
            Code::PHA => instructions::stack::pha(cpu.bus, cpu.registers),
            Code::PHP => instructions::stack::php(cpu.bus, cpu.registers),
            Code::PLA => instructions::stack::pla(cpu.bus, cpu.registers),
            Code::PLP => instructions::stack::plp(cpu.bus, cpu.registers),
            // -- Logical --
            Code::AND => instructions::logical::and(cpu.bus, cpu.registers, &opecode.mode),
            Code::EOR => instructions::logical::eor(cpu.bus, cpu.registers, &opecode.mode),
            Code::ORA => instructions::logical::ora(cpu.bus, cpu.registers, &opecode.mode),
            Code::BIT => instructions::logical::bit(cpu.bus, cpu.registers, &opecode.mode),
            // -- Arithmetic --
            // Code::ADC
            // Code::SBC
            // Code::CMP
            // Code::CPX
            // Code::CPY
            // -- Increment --
            Code::INC => instructions::increment::inc(cpu.bus, cpu.registers, &opecode.mode),
            Code::INX => instructions::increment::inx(cpu.registers),
            Code::INY => instructions::increment::iny(cpu.registers),
            // -- Decrement --
            Code::DEC => instructions::decrement::dec(cpu.bus, cpu.registers, &opecode.mode),
            Code::DEX => instructions::decrement::dex(cpu.registers),
            Code::DEY => instructions::decrement::dey(cpu.registers),
            // -- Shift --
            // Code::ASL
            // Code::LSR
            // Code::ROL
            // Code::ROR
            // -- Jump --
            Code::JMP => instructions::jump::jmp(cpu.bus, cpu.registers, &opecode.mode),
            Code::JSR => instructions::jump::jsr(cpu.bus, cpu.registers, &opecode.mode),
            Code::RTS => instructions::jump::rts(cpu.bus, cpu.registers),
            // -- Branches --
            Code::BCC => instructions::branch::bcc(cpu.bus, cpu.registers, &opecode.mode),
            Code::BCS => instructions::branch::bcs(cpu.bus, cpu.registers, &opecode.mode),
            Code::BEQ => instructions::branch::beq(cpu.bus, cpu.registers, &opecode.mode),
            Code::BMI => instructions::branch::bmi(cpu.bus, cpu.registers, &opecode.mode),
            Code::BNE => instructions::branch::bne(cpu.bus, cpu.registers, &opecode.mode),
            Code::BPL => instructions::branch::bpl(cpu.bus, cpu.registers, &opecode.mode),
            Code::BVC => instructions::branch::bvc(cpu.bus, cpu.registers, &opecode.mode),
            Code::BVS => instructions::branch::bvs(cpu.bus, cpu.registers, &opecode.mode),
            // -- Flags --
            Code::CLC => instructions::flags::clc(cpu.registers),
            Code::CLD => instructions::flags::cld(cpu.registers),
            Code::CLI => instructions::flags::cli(cpu.registers),
            Code::CLV => instructions::flags::clv(cpu.registers),
            Code::SEC => instructions::flags::sec(cpu.registers),
            Code::SED => instructions::flags::sed(cpu.registers),
            Code::SEI => instructions::flags::sei(cpu.registers),
            // -- System --
            Code::BRK => instructions::system::brk(cpu.bus, cpu.registers),
            Code::NOP => { /* NOP do nothing */ },
            Code::RTI => instructions::system::rti(cpu.bus, cpu.registers),
        };

        opecode.cycle
    }
}

#[cfg(test)]
mod cpu_tests {
    use crate::nes::{
        bus::Bus,
        cpu::{fetch, registers::CpuRegisters},
    };

    use super::Cpu;

    struct MockBus {
        data: Vec<u8>,
    }

    impl Bus for MockBus {
        fn read(&self, address: u16) -> u8 {
            self.data[address as usize]
        }

        fn read_u16(&self, _address: u16) -> u16 {
            unimplemented!()
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
