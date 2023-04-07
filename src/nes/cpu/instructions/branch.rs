use crate::nes::{cpu::{registers::{CpuRegisters, CpuStatusFlag}, opecode::AddressingMode, fetch}, bus::Bus};

pub fn bne<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) where T: Bus {
  let address = fetch::read_operand_address(bus, registers, mode);

  if !registers.p.contains(CpuStatusFlag::ZERO) {
      registers.pc = address;
  }
}

#[cfg(test)]
mod branch_tests {
  use crate::nes::{cpu::{registers::{CpuRegisters, CpuStatusFlag}, opecode::AddressingMode}, bus::Bus};

  use super::*;

  struct MockBus {
    data: Vec<u8>,
  }

  impl MockBus {
    fn new() -> Self {
      Self { data: vec![0; 0xFFFF] }
    }
  }

  impl Bus for MockBus {
    fn read(&self, address: u16) -> u8 {
      self.data[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
      self.data[address as usize] = data;
    }
  }

  mod bne {
    use super::*;

    #[test]
    fn bne_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0x50);
      registers.pc = 0x0005;
      registers.p.remove(CpuStatusFlag::ZERO);

      bne(&mut bus, &mut registers, &AddressingMode::Relative);

      // Program counter should be incremented by 1 + 0x0050
      let expect = 0x0005 + 1 + 0x0050;

      assert_eq!(registers.pc, expect);
    }

    #[test]
    fn bne_not_set_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0x50);
      registers.pc = 0x0005;
      registers.p.insert(CpuStatusFlag::ZERO);

      bne(&mut bus, &mut registers, &AddressingMode::Relative);

      // Program counter should be incremented by 1
      let expect = 0x0005 + 1;

      assert_eq!(registers.pc, expect);
    }
  }
}
