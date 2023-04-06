use crate::nes::{cpu::{registers::CpuRegisters}};

pub fn dey(registers: &mut CpuRegisters) {
  registers.y = registers.y.wrapping_sub(1);
  registers.update_zero_and_negative_flags(registers.y);
}
