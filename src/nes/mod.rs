use core::time;
use std::{fs::File, io::Read, thread};

use self::{cartridge::Cartridge, cpu::{cpu::Cpu, registers::CpuRegisters, bus::CpuBus}, ram::Ram, ppu::{registers::PpuRegisters, bus::PpuBus, frame::Frame, pattern_table::PatternTable}};
use anyhow::Result;

pub mod cartridge;
pub mod ppu;
pub mod ram;
pub mod cpu;
pub mod bus;

const WRAM_SIZE: u16 = 2024;
const VRAM_SIZE: u16 = 2024;

pub struct Nes<T> {
    cartridge: Cartridge,
    cpu_registers: CpuRegisters,
    ppu_registers: T,
    wram: Ram,
    vram: Ram,
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

        Ok(Nes { cartridge, cpu_registers, ppu_registers, wram, vram })
    }

    pub fn run<'call, Fr, Fi>(&mut self, mut render_callback: Fr, mut input_callback: Fi)
    where
        Fr: FnMut(&Frame) + 'call,
        Fi: FnMut() + 'call,
    {
        let mut pattern_table = PatternTable::new(Ram::from_vec(self.cartridge.character_rom.clone())).unwrap();
        let mut ppu_bus = PpuBus::new(&mut self.ppu_registers, &mut pattern_table, &mut self.vram);
        let mut cpu_bus = CpuBus::new(&self.cartridge.program_rom, &mut self.wram, &mut ppu_bus);
        let mut cpu = Cpu::new(&mut self.cpu_registers, &mut cpu_bus);

        loop {
            cpu.run();

            let mut frame = Frame::new();
            frame.set_pixel(10, 10, (255, 255, 255));
            frame.set_pixel(11, 10, (255, 255, 255));
            frame.set_pixel(12, 10, (255, 255, 255));

            input_callback();
            render_callback(&frame);

            let ten_millis = time::Duration::from_millis(50);
            thread::sleep(ten_millis);
        }
    }
}
