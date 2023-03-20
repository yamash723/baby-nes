use std::{cell::RefCell};

use crate::nes::{ram::Ram, bus::Bus};

use super::{registers::{PpuRegistration, PpuRegisters}, ppu::PpuContext, palette_ram::PaletteRam};

pub struct PpuBus<'a> {
    pub registers: &'a mut PpuRegisters,
    pub context: RefCell<PpuContext<'a>>,
}

impl <'a> PpuBus<'a> {
    pub fn new(registers: &'a mut PpuRegisters , pattern_table: &'a mut Ram, vram: &'a mut Ram) -> Self {
        let context = PpuContext {
            pattern_table,
            vram,
            palette_ram: PaletteRam::new(),
        };

        Self { registers, context: RefCell::new(context) }
    }
}

impl <'a> Bus for PpuBus<'a> {
    fn read(&self, address: u16) -> u8 {
        self.registers.read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.registers.write(address, data, &mut self.context.borrow_mut());
    }
}
