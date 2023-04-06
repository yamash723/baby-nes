use crate::nes::{cpu::{registers::CpuRegisters, opecode::AddressingMode, fetch}, bus::Bus};

pub fn jmp<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) where T: Bus {
  registers.pc = fetch::read_operand_address(bus, registers, mode);
}
