use crate::nes::{cpu::{registers::{CpuRegisters, CpuStatusFlag}, opecode::AddressingMode, fetch}, bus::Bus};

pub fn bne<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) where T: Bus {
  let address = fetch::read_operand_address(bus, registers, mode);

  if !registers.p.contains(CpuStatusFlag::ZERO) {
      registers.pc = address;
  }
}
