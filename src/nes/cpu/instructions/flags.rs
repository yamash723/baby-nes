use crate::nes::{cpu::{registers::{CpuRegisters, CpuStatusFlag}}};

pub fn sei(registers: &mut CpuRegisters) {
  registers.p.insert(CpuStatusFlag::INTERRUPT_DISABLE);
}

#[cfg(test)]
mod flags_tests {
  use super::*;

  #[test]
  fn sei_test() {
    let mut registers = CpuRegisters::new();
    registers.p.remove(CpuStatusFlag::INTERRUPT_DISABLE);

    sei(&mut registers);

    assert!(registers.p.contains(CpuStatusFlag::INTERRUPT_DISABLE));
  }
}
