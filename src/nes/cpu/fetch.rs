use crate::nes::bus::Bus;

use super::{opecode::AddressingMode, registers::CpuRegisters};

pub fn fetch<T>(bus: &mut T, registers: &mut CpuRegisters) -> u8
where
    T: Bus,
{
    let data = bus.read(registers.pc);
    registers.pc += 1;
    data
}

pub fn fetch_relative<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    let offset = fetch(bus, registers) as u16;

    if offset < 0x80 {
        registers.pc + offset
    } else {
        registers.pc + offset - 0x100
    }
}

pub fn fetch_absolute<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    let lower = fetch(bus, registers) as u16;
    let upper = fetch(bus, registers) as u16;
    lower | upper << 8
}

pub fn fetch_absolute_x<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    fetch_absolute(bus, registers) + registers.x as u16
}

// ToDo: read_operand_addressとの使い分けが微妙
pub fn fetch_operand<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) -> u8
where
    T: Bus,
{
    if mode == &AddressingMode::Immediate {
        return fetch(bus, registers);
    }

    let address = read_operand_address(bus, registers, mode);
    bus.read(address)
}

pub fn read_operand_address<T>(
    bus: &mut T,
    registers: &mut CpuRegisters,
    mode: &AddressingMode,
) -> u16
where
    T: Bus,
{
    match mode {
        AddressingMode::Immediate => fetch(bus, registers) as u16,
        AddressingMode::Absolute => fetch_absolute(bus, registers),
        AddressingMode::AbsoluteIndexedX => fetch_absolute_x(bus, registers),
        AddressingMode::Relative => fetch_relative(bus, registers),
        _ => todo!(),
    }
}
