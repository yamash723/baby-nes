use core::time;
use std::{fs::File, io::Read, thread};

use self::{cartridge::Cartridge, cpu::{cpu::Cpu, registers::Registers, bus::CpuBus}, ram::Ram};
use anyhow::Result;

pub mod cartridge;
pub mod ppu;
pub mod ram;
pub mod cpu;
pub mod bus;

const WRAM_SIZE: u16 = 2024;

pub struct Nes {
    cartridge: Cartridge,
    cpu_registers: Registers,
    wram: Ram,
}

impl Nes {
    pub fn new(path: &str) -> Result<Self> {
        // ToDo
        let mut f = File::open(path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let cartridge = Cartridge::new(&buffer).unwrap();
        let cpu_registers = Registers::new();
        let wram = Ram::new(WRAM_SIZE);

        Ok(Nes { cartridge, cpu_registers, wram })
    }

    pub fn run(&mut self) {
        let mut bus = CpuBus::new(&self.cartridge.program_rom, &mut self.wram);
        let mut cpu = Cpu::new(&mut self.cpu_registers, &mut bus);

        loop {
            cpu.run();

            let ten_millis = time::Duration::from_millis(50);
            thread::sleep(ten_millis);
        }
    }
}