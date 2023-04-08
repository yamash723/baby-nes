use crate::nes::{cpu::{opecode::AddressingMode, fetch, registers::CpuRegisters}, bus::Bus};

pub fn sta<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) where T: Bus {
  let address = fetch::read_operand_address(bus, registers, mode);
  bus.write(address, registers.a);
}

pub fn stx<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) where T: Bus {
  unimplemented!();
}

pub fn sty<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) where T: Bus {
  unimplemented!();
}

#[cfg(test)]
mod store_tests {
  use crate::nes::cpu::{instructions::instructions_test::MockBus};
  use super::*;

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

    assert_eq!(registers.a, 0xFF);
  }
}
