use crate::nes::cpu::registers::{CpuRegisters, CpuStatusFlag};

pub fn clc(registers: &mut CpuRegisters) {
    registers.p.remove(CpuStatusFlag::CARRY);
}

pub fn cld(registers: &mut CpuRegisters) {
    registers.p.remove(CpuStatusFlag::DECIMAL);
}

pub fn cli(registers: &mut CpuRegisters) {
    registers.p.remove(CpuStatusFlag::INTERRUPT_DISABLE);
}

pub fn clv(registers: &mut CpuRegisters) {
    registers.p.remove(CpuStatusFlag::OVERFLOW);
}

pub fn sec(registers: &mut CpuRegisters) {
    registers.p.insert(CpuStatusFlag::CARRY);
}

pub fn sed(registers: &mut CpuRegisters) {
    registers.p.insert(CpuStatusFlag::DECIMAL);
}

pub fn sei(registers: &mut CpuRegisters) {
    registers.p.insert(CpuStatusFlag::INTERRUPT_DISABLE);
}

#[cfg(test)]
mod flags_tests {
    use super::*;

    #[test]
    fn clc_test() {
        let mut registers = CpuRegisters::new();
        registers.p.insert(CpuStatusFlag::CARRY);

        clc(&mut registers);

        assert!(!registers.p.contains(CpuStatusFlag::CARRY));
    }

    #[test]
    fn cld_test() {
        let mut registers = CpuRegisters::new();
        registers.p.insert(CpuStatusFlag::DECIMAL);

        cld(&mut registers);

        assert!(!registers.p.contains(CpuStatusFlag::DECIMAL));
    }

    #[test]
    fn cli_test() {
        let mut registers = CpuRegisters::new();
        registers.p.insert(CpuStatusFlag::INTERRUPT_DISABLE);

        cli(&mut registers);

        assert!(!registers.p.contains(CpuStatusFlag::INTERRUPT_DISABLE));
    }

    #[test]
    fn clv_test() {
        let mut registers = CpuRegisters::new();
        registers.p.insert(CpuStatusFlag::OVERFLOW);

        clv(&mut registers);

        assert!(!registers.p.contains(CpuStatusFlag::OVERFLOW));
    }

    #[test]
    fn sec_test() {
        let mut registers = CpuRegisters::new();
        registers.p.remove(CpuStatusFlag::CARRY);

        sec(&mut registers);

        assert!(registers.p.contains(CpuStatusFlag::CARRY));
    }

    #[test]
    fn sed_test() {
        let mut registers = CpuRegisters::new();
        registers.p.remove(CpuStatusFlag::DECIMAL);

        sed(&mut registers);

        assert!(registers.p.contains(CpuStatusFlag::DECIMAL));
    }

    #[test]
    fn sei_test() {
        let mut registers = CpuRegisters::new();
        registers.p.remove(CpuStatusFlag::INTERRUPT_DISABLE);

        sei(&mut registers);

        assert!(registers.p.contains(CpuStatusFlag::INTERRUPT_DISABLE));
    }
}
