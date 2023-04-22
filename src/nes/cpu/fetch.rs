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

pub fn fetch_zero_page_x<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    let address = fetch(bus, registers).wrapping_add(registers.x);
    address as u16
}

pub fn fetch_zero_page_y<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    let address = fetch(bus, registers).wrapping_add(registers.y);
    address as u16
}

pub fn fetch_absolute<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    let lower = fetch(bus, registers);
    let upper = fetch(bus, registers);
    u16::from_be_bytes([upper, lower])
}

pub fn fetch_absolute_x<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    fetch_absolute(bus, registers) + registers.x as u16
}

pub fn fetch_absolute_y<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    fetch_absolute(bus, registers) + registers.y as u16
}

pub fn fetch_indexed_indirect<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    let address = u16::from_be_bytes([0x00, fetch(bus, registers).wrapping_add(registers.x)]);
    let upper = bus.read(address);
    let lower = bus.read(address + 1);
    u16::from_be_bytes([upper, lower])
}

pub fn fetch_indirect_indexed<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    let address = u16::from_be_bytes([0x00, fetch(bus, registers)]);
    let upper = bus.read(address);
    let lower = bus.read(address + 1);
    u16::from_be_bytes([upper, lower]) + registers.y as u16
}

pub fn fetch_absolute_indirect<T>(bus: &mut T, registers: &mut CpuRegisters) -> u16
where
    T: Bus,
{
    let address = fetch_absolute(bus, registers);
    let upper = bus.read(address);
    let lower = bus.read(address + 1);
    u16::from_be_bytes([upper, lower])
}

// ToDo: read_operand_addressとの使い分けが微妙
pub fn fetch_operand<T>(bus: &mut T, registers: &mut CpuRegisters, mode: &AddressingMode) -> u8
where
    T: Bus,
{
    if mode == &AddressingMode::Implied || mode == &AddressingMode::Accumulator {
        return 0x00;
    }

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
        AddressingMode::Implied => 0x0000,
        AddressingMode::Accumulator => 0x0000,
        AddressingMode::Immediate => fetch(bus, registers) as u16,
        AddressingMode::ZeroPage => fetch(bus, registers) as u16,
        AddressingMode::ZeroPageIndexedX => fetch_zero_page_x(bus, registers) as u16,
        AddressingMode::ZeroPageIndexedY => fetch_zero_page_y(bus, registers) as u16,
        AddressingMode::Absolute => fetch_absolute(bus, registers),
        AddressingMode::AbsoluteIndexedX => fetch_absolute_x(bus, registers),
        AddressingMode::AbsoluteIndexedY => fetch_absolute_y(bus, registers),
        AddressingMode::Relative => fetch_relative(bus, registers),
        AddressingMode::IndexedIndirect => fetch_indexed_indirect(bus, registers),
        AddressingMode::IndirectIndexed => fetch_indirect_indexed(bus, registers),
        AddressingMode::AbsoluteIndirect => fetch_absolute_indirect(bus, registers),
    }
}
