use crate::nes::{cpu::{registers::{CpuRegisters, CpuStatusFlag}, opecode::AddressingMode, fetch}, bus::Bus};

pub fn bne<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) where T: Bus {
  let address = fetch::read_operand_address(bus, registers, mode);

  if !registers.p.contains(CpuStatusFlag::ZERO) {
      registers.pc = address;
  }
}

#[cfg(test)]
mod branch_tests {
  use crate::nes::{cpu::{registers::{CpuRegisters, CpuStatusFlag}, opecode::AddressingMode, instructions::instructions_test::MockBus}, bus::Bus};

  use super::*;

  #[test]
  fn bne_test() {
    struct State {
      pub pc: u16,
      pub data: u8,
      pub is_set_zero_flg: bool,
      pub expect_pc: u16,
    }

    let patterns = vec![
      // BNE should be executed when zero flag is not set. (increment pc & add data)
      State { pc: 0x0005, data: 0x50, is_set_zero_flg: false, expect_pc: 0x0005 + 1 + 0x0050 },
      // BNE should not be executed when zero flag is set. (increment pc only)
      State { pc: 0x0005, data: 0x50, is_set_zero_flg: true, expect_pc: 0x0005 + 1 },
    ];

    for state in patterns {
      let mut bus = MockBus::new();
      let mut registers = CpuRegisters::new();

      bus.write(state.pc, state.data);
      registers.pc = state.pc;
      registers.p.set(CpuStatusFlag::ZERO, state.is_set_zero_flg);

      bne(&mut bus, &mut registers, &AddressingMode::Relative);

      assert_eq!(registers.pc, state.expect_pc);
    }
  }
}
