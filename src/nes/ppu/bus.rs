use std::{cell::RefCell};

use crate::nes::{ram::Ram, bus::Bus};

use super::{registers::PpuRegistration, ppu::PpuContext};

pub struct PpuBus<'a, T: PpuRegistration> {
    pub ppu_registers: &'a mut T,
    pub ppu_context: RefCell<PpuContext<'a>>,
}

impl <'a, T> PpuBus<'a, T> where T: PpuRegistration {
    pub fn new(ppu_registers: &'a mut T, pattern_table: &'a mut Vec<u8>, vram: &'a mut Ram, palette_ram: &'a mut Ram) -> Self {
        let ppu_context = PpuContext { pattern_table, vram, palette_ram };
        Self { ppu_registers, ppu_context: RefCell::new(ppu_context) }
    }
}

impl <'a, T> Bus for PpuBus<'a, T> where T: PpuRegistration {
    fn read(&self, address: u16) -> u8 {
        self.ppu_registers.read(address)
    }

    fn write(&mut self, address: u16, data: u8) {
        self.ppu_registers.write(address, data, &mut self.ppu_context.borrow_mut());
    }
}