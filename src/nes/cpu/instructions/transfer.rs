use crate::nes::{cpu::{registers::CpuRegisters}};

pub fn txs(registers: &mut CpuRegisters) {
  registers.s = registers.x;
}

#[cfg(test)]
mod transfer_tests {
  use super::*;

  #[test]
  fn txs_test() {
    let mut registers = CpuRegisters::new();
    registers.x = 0x10;

    txs(&mut registers);

    assert_eq!(registers.s, 0x10);
  }
}
