use core::time;
use std::{fs::File, io::Read, thread};

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

const WRAM_SIZE: u16 = 2024;
const VRAM_SIZE: u16 = 2024;

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

                    let background = &self.ppu.background;
                    let mut frame = Frame::new();

                    for tile in background.iter() {
                        let sprite = &tile.sprite;
                        let position = &tile.position;

                        let start_pos_x = position.x as usize * 8;
                        let start_pos_y = position.y as usize * 8;

                        for (y, sprite_line) in sprite.to_vec().iter().enumerate() {
                            for (x, palette_number) in sprite_line.iter().enumerate() {
                                let point_x = start_pos_x + x;
                                let point_y = start_pos_y + y;
                                let palette = tile.palettes.get(*palette_number as usize);
                                let color = NES_COLORS[palette.get_palette_number() as usize];

                                frame.set_pixel(point_x, point_y, (color[0], color[1], color[2]));
                            }
                        }
                    }

                    println!("Background count: {:?}", background.iter().len());
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

static NES_COLORS: [[u8; 3]; 64] = [
    [0x80, 0x80, 0x80],
    [0x00, 0x3D, 0xA6],
    [0x00, 0x12, 0xB0],
    [0x44, 0x00, 0x96],
    [0xA1, 0x00, 0x5E],
    [0xC7, 0x00, 0x28],
    [0xBA, 0x06, 0x00],
    [0x8C, 0x17, 0x00],
    [0x5C, 0x2F, 0x00],
    [0x10, 0x45, 0x00],
    [0x05, 0x4A, 0x00],
    [0x00, 0x47, 0x2E],
    [0x00, 0x41, 0x66],
    [0x00, 0x00, 0x00],
    [0x05, 0x05, 0x05],
    [0x05, 0x05, 0x05],
    [0xC7, 0xC7, 0xC7],
    [0x00, 0x77, 0xFF],
    [0x21, 0x55, 0xFF],
    [0x82, 0x37, 0xFA],
    [0xEB, 0x2F, 0xB5],
    [0xFF, 0x29, 0x50],
    [0xFF, 0x22, 0x00],
    [0xD6, 0x32, 0x00],
    [0xC4, 0x62, 0x00],
    [0x35, 0x80, 0x00],
    [0x05, 0x8F, 0x00],
    [0x00, 0x8A, 0x55],
    [0x00, 0x99, 0xCC],
    [0x21, 0x21, 0x21],
    [0x09, 0x09, 0x09],
    [0x09, 0x09, 0x09],
    [0xFF, 0xFF, 0xFF],
    [0x0F, 0xD7, 0xFF],
    [0x69, 0xA2, 0xFF],
    [0xD4, 0x80, 0xFF],
    [0xFF, 0x45, 0xF3],
    [0xFF, 0x61, 0x8B],
    [0xFF, 0x88, 0x33],
    [0xFF, 0x9C, 0x12],
    [0xFA, 0xBC, 0x20],
    [0x9F, 0xE3, 0x0E],
    [0x2B, 0xF0, 0x35],
    [0x0C, 0xF0, 0xA4],
    [0x05, 0xFB, 0xFF],
    [0x5E, 0x5E, 0x5E],
    [0x0D, 0x0D, 0x0D],
    [0x0D, 0x0D, 0x0D],
    [0xFF, 0xFF, 0xFF],
    [0xA6, 0xFC, 0xFF],
    [0xB3, 0xEC, 0xFF],
    [0xDA, 0xAB, 0xEB],
    [0xFF, 0xA8, 0xF9],
    [0xFF, 0xAB, 0xB3],
    [0xFF, 0xD2, 0xB0],
    [0xFF, 0xEF, 0xA6],
    [0xFF, 0xF7, 0x9C],
    [0xD7, 0xE8, 0x95],
    [0xA6, 0xED, 0xAF],
    [0xA2, 0xF2, 0xDA],
    [0x99, 0xFF, 0xFC],
    [0xDD, 0xDD, 0xDD],
    [0x11, 0x11, 0x11],
    [0x11, 0x11, 0x11],
];
