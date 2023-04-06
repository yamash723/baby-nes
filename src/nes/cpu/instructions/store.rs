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
