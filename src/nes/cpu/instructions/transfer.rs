use crate::nes::{cpu::{registers::CpuRegisters}};

pub fn txs(registers: &mut CpuRegisters) {
  registers.s = registers.a;
}
