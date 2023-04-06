use crate::nes::{cpu::{registers::CpuRegisters}};

pub fn inx(registers: &mut CpuRegisters) {
  registers.x = registers.x.wrapping_add(1);
  registers.update_zero_and_negative_flags(registers.x);
}
