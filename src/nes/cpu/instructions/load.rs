use crate::nes::{cpu::{opecode::AddressingMode, fetch, registers::CpuRegisters}, bus::Bus};

pub fn lda<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) where T: Bus {
  registers.a = fetch::fetch_opecode(bus, registers, mode);
  registers.update_zero_and_negative_flags(registers.a);
}

pub fn ldx<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) where T: Bus {
  registers.x = fetch::fetch_opecode(bus, registers, mode);
  registers.update_zero_and_negative_flags(registers.x);
}

pub fn ldy<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) where T: Bus {
  registers.y = fetch::fetch_opecode(bus, registers, mode);
  registers.update_zero_and_negative_flags(registers.y);
}

#[cfg(test)]
mod store_tests {
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

  mod lda {
    use super::*;

    #[test]
    fn lda_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0x01);
      registers.pc = 0x0005;

      lda(&mut bus, &mut registers, &AddressingMode::Immediate);

      assert_eq!(registers.a, 0x01);
      assert!(!registers.p.contains(CpuStatusFlag::ZERO));
      assert!(!registers.p.contains(CpuStatusFlag::NEGATIVE));
    }

    #[test]
    fn lda_zero_flag_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0x00);
      registers.pc = 0x0005;

      lda(&mut bus, &mut registers, &AddressingMode::Immediate);

      assert_eq!(registers.a, 0x00);
      assert!(registers.p.contains(CpuStatusFlag::ZERO));
      assert!(!registers.p.contains(CpuStatusFlag::NEGATIVE));
    }

    #[test]
    fn lda_negative_flag_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0xFF);
      registers.pc = 0x0005;

      lda(&mut bus, &mut registers, &AddressingMode::Immediate);

      assert_eq!(registers.a, 0xFF);
      assert!(!registers.p.contains(CpuStatusFlag::ZERO));
      assert!(registers.p.contains(CpuStatusFlag::NEGATIVE));
    }
  }

  mod ldx {
    use super::*;

    #[test]
    fn ldx_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0x01);
      registers.pc = 0x0005;

      ldx(&mut bus, &mut registers, &AddressingMode::Immediate);

      assert_eq!(registers.x, 0x01);
      assert!(!registers.p.contains(CpuStatusFlag::ZERO));
      assert!(!registers.p.contains(CpuStatusFlag::NEGATIVE));
    }

    #[test]
    fn ldx_zero_flag_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0x00);
      registers.pc = 0x0005;

      ldx(&mut bus, &mut registers, &AddressingMode::Immediate);

      assert_eq!(registers.x, 0x00);
      assert!(registers.p.contains(CpuStatusFlag::ZERO));
      assert!(!registers.p.contains(CpuStatusFlag::NEGATIVE));
    }

    #[test]
    fn ldx_negative_flag_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0xFF);
      registers.pc = 0x0005;

      ldx(&mut bus, &mut registers, &AddressingMode::Immediate);

      assert_eq!(registers.x, 0xFF);
      assert!(!registers.p.contains(CpuStatusFlag::ZERO));
      assert!(registers.p.contains(CpuStatusFlag::NEGATIVE));
    }
  }

  mod ldy {
    use super::*;

    #[test]
    fn ldy_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0x01);
      registers.pc = 0x0005;

      ldy(&mut bus, &mut registers, &AddressingMode::Immediate);

      assert_eq!(registers.y, 0x01);
      assert!(!registers.p.contains(CpuStatusFlag::ZERO));
      assert!(!registers.p.contains(CpuStatusFlag::NEGATIVE));
    }

    #[test]
    fn ldy_zero_flag_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0x00);
      registers.pc = 0x0005;

      ldy(&mut bus, &mut registers, &AddressingMode::Immediate);

      assert_eq!(registers.y, 0x00);
      assert!(registers.p.contains(CpuStatusFlag::ZERO));
      assert!(!registers.p.contains(CpuStatusFlag::NEGATIVE));
    }

    #[test]
    fn ldy_negative_flag_test() {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(0x0005, 0xFF);
      registers.pc = 0x0005;

      ldy(&mut bus, &mut registers, &AddressingMode::Immediate);

      assert_eq!(registers.y, 0xFF);
      assert!(!registers.p.contains(CpuStatusFlag::ZERO));
      assert!(registers.p.contains(CpuStatusFlag::NEGATIVE));
    }
  }
}
