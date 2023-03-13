use core::time;
use std::{fs::File, io::Read, thread};

use self::{cartridge::Cartridge, cpu::{cpu::Cpu, registers::CpuRegisters, bus::CpuBus}, ram::Ram, ppu::{registers::PpuRegisters, bus::PpuBus}};
use anyhow::Result;

pub mod cartridge;
pub mod ppu;
pub mod ram;
pub mod cpu;
pub mod bus;

const WRAM_SIZE: u16 = 2024;
const VRAM_SIZE: u16 = 2024;
const PALETTE_TABLE_SIZE: u16 = 32;

pub struct Nes<T> {
    cartridge: Cartridge,
    cpu_registers: CpuRegisters,
    ppu_registers: T,
    wram: Ram,
    vram: Ram,
    palette_ram: Ram,
}

impl Nes<PpuRegisters> {
    pub fn new(path: &str) -> Result<Self> {
        // ToDo
        let mut f = File::open(path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let cartridge = Cartridge::new(&buffer).unwrap();
        let cpu_registers = CpuRegisters::new();
        let ppu_registers = PpuRegisters::new();
        let wram = Ram::new(WRAM_SIZE);
        let vram = Ram::new(VRAM_SIZE);
        let palette_ram = Ram::new(PALETTE_TABLE_SIZE);

        Ok(Nes { cartridge, cpu_registers, ppu_registers, wram, vram, palette_ram })
    }

    pub fn run(&mut self) {
        let mut ppu_bus = PpuBus::new(&mut self.ppu_registers, &mut self.cartridge.character_rom, &mut self.vram, &mut self.palette_ram);
        let mut cpu_bus = CpuBus::new(&self.cartridge.program_rom, &mut self.wram, &mut ppu_bus);
        let mut cpu = Cpu::new(&mut self.cpu_registers, &mut cpu_bus);

        loop {
            cpu.run();

            let ten_millis = time::Duration::from_millis(50);
            thread::sleep(ten_millis);
        }
    }
}
