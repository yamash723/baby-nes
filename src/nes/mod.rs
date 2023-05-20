use core::time;
use std::{fs::File, io::Read, thread};

use crate::nes::ppu::render::rendering_frame;

use self::{
    cartridge::Cartridge,
    cpu::{bus::CpuBus, cpu::Cpu, registers::CpuRegisters},
    ppu::{
        frame::Frame,
        pattern_table::PatternTable,
        ppu::{Ppu, PpuRunResult},
    },
    ram::Ram,
};
use anyhow::Result;

pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod ppu;
pub mod ram;

const WRAM_SIZE: u16 = 2028;
const VRAM_SIZE: u16 = 2028;

pub struct Nes {
    cartridge: Cartridge,
    cpu_registers: CpuRegisters,
    ppu: Ppu,
    wram: Ram,
}

impl Nes {
    pub fn new(path: &str) -> Result<Self> {
        // ToDo
        let mut f = File::open(path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        let cartridge = Cartridge::new(&buffer).unwrap();
        let cpu_registers = CpuRegisters::new();
        let wram = Ram::new(WRAM_SIZE);

        let pattern_table =
            PatternTable::new(Ram::from_vec(cartridge.character_rom.clone())).unwrap();
        let vram = Ram::new(VRAM_SIZE);
        let ppu = Ppu::new(pattern_table, vram);

        Ok(Nes {
            cartridge,
            cpu_registers,
            ppu,
            wram,
        })
    }

    pub fn run<'call, Fr, Fi>(&mut self, mut render_callback: Fr, mut input_callback: Fi)
    where
        Fr: FnMut(&Frame) + 'call,
        Fi: FnMut() + 'call,
    {
        loop {
            let cycle = {
                let mut cpu_bus =
                    CpuBus::new(&self.cartridge.program_rom, &mut self.wram, &mut self.ppu);
                Cpu::run(&mut self.cpu_registers, &mut cpu_bus)
            };

            // println!("PPU context cycle: {:?} / line: {:?}", self.ppu.cycle, self.ppu.line);

            match self.ppu.run(cycle * 3) {
                PpuRunResult::FinishedBuildAllBackgroundLine => {
                    println!("Finished build all background line.");
                    let frame = rendering_frame(&self.ppu);
                    render_callback(&frame);
                    self.ppu.reset_background();

                    let ten_millis = time::Duration::from_millis(1000);
                    thread::sleep(ten_millis);
                }
                _ => {}
            };

            input_callback();
        }
    }
}
