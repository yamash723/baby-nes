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
