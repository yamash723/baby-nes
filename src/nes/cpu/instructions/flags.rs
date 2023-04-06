use crate::nes::{cpu::{registers::{CpuRegisters, CpuStatusFlag}}};

pub fn sei(registers: &mut CpuRegisters) {
  registers.p.insert(CpuStatusFlag::INTERRUPT_DISABLE);
}
