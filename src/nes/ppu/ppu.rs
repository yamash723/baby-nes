// Must be refactor

use crate::nes::{ram::Ram, bus::Bus};

use super::{palette_ram::PaletteRam, background::Background, registers::{PpuRegisters, PpuRegistration}, bus::PpuBus, tile_position::TilePosition};

pub struct Ppu<'a> {
    pub cycle: usize,
    pub line: usize,
    pub background: Background,
    pub ppu_bus: &'a mut PpuBus<'a>,
}

pub struct PpuContext<'a> {
    pub pattern_table: &'a mut Ram,
    pub vram: &'a mut Ram,
    pub palette_ram: PaletteRam,
}

const CLOCK_TO_RENDER_LINE: usize = 341;

pub enum PpuRunResult {
    CountUpCycle,
    FinishedBuildBackgroundLine,
    FinishedBuildAllBackgroundLine,
}

impl <'a> Ppu<'a> {
    pub fn new(ppu_bus: &mut PpuBus) -> Self {
        Ppu {
            cycle: 0,
            line: 0,
            background: Background::new(),
            ppu_bus: ppu_bus,
        }
    }

    pub fn read(&'a mut self, addr: u16) -> u8 {
        // self.ppu_bus.registers.read(addr, &mut self.context)
        self.ppu_bus.read(addr)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        // self.ppu_bus.registers.write(addr, data, &mut self.context);
        self.ppu_bus.write(addr, data);
    }

    pub fn run(&mut self, cycle: usize) -> PpuRunResult {
        self.cycle += cycle;

        if self.cycle < CLOCK_TO_RENDER_LINE {
            return PpuRunResult::CountUpCycle;
        }

        self.cycle -= CLOCK_TO_RENDER_LINE;
        self.line += 1;

        // is need building a background line.
        if self.line <= 240 && self.line % 8 == 0 {
            /*
                The name table has such a structure.

                nametable: has 4 nametable. (ID: 0-3)
                x: 32 tiles(8x8 pixel) in 256 pixel.
                y: 30 tiles(8x8 pixel) in 240 pixel.
                +------------+------------+
                |            |            |
                |  0(0x2000) |  1(0x2400) |
                |            |            |
                +------------+------------+
                |            |            |
                |  2(0x2800) |  3(0x2C00) |
                |            |            |
                +------------+------------+

                name table: 0, 2 -> start x position is 0.
                name table: 1, 3 -> start x position is 32.
            */

            // borrowで無理やりやってるので治す
            let nametable_id = self.ppu_bus.registers.get_nametable_address();
            let pos_x_start = (nametable_id.into() % 2) * 32;
            let pos_x_end = pos_x_start + 32;
            let pos_y = (self.line / 8) as u8;

            for pos_x in pos_x_start..pos_x_end {
                let tile_pos = TilePosition::new(pos_x, pos_y);
                let tile = Tile::build(tile_pos, &self.context);
                self.background.push(tile);
            }
        }

        // is not finished building all the background lines.
        if self.line < 262 {
            return PpuRunResult::FinishedBuildBackgroundLine;
        }

        self.line = 0;
        PpuRunResult::FinishedBuildAllBackgroundLine
    }
}
