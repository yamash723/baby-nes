// Must be refactor

use super::{
    background::Background,
    palette_ram::{PaletteRam, PaletteType},
    pattern_table::PatternTable,
    registers::{PpuRegisters, PpuRegistration, ppu_status::PpuStatus},
    render::RenderContext,
    sprite::{build_sprite, Sprite},
    tile::Tile,
    tile_position::TilePosition,
};
use crate::nes::{
    ppu::registers::{ppu_control::PpuCtrl, ppu_mask::PpuMask},
    ram::Ram,
};
use anyhow::Result;

pub struct Ppu {
    pub cycle: u16,
    pub line: u16,
    pub background: Background,
    pub pattern_table: PatternTable,
    pub vram: Ram,
    pub palette_ram: PaletteRam,
    pub ppu_registers: PpuRegisters,
}

const CLOCK_TO_RENDER_LINE: u16 = 341;

pub enum PpuRunResult {
    CountUpCycle,
    FinishedBuildBackgroundLine,
    FinishedBuildAllBackgroundLine,
}

impl Ppu {
    pub fn new(pattern_table: PatternTable, vram: Ram) -> Self {
        Self {
            cycle: 0,
            line: 0,
            background: Background::new(),
            pattern_table,
            vram,
            palette_ram: PaletteRam::new(),
            ppu_registers: PpuRegisters::new(),
        }
    }

    pub fn run(&mut self, cycle: u16) -> PpuRunResult {
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
            let nametable_id = self.ppu_registers.get_nametable_address();
            let pos_x_start = (nametable_id as u16 % 2) * 32;
            let pos_x_end = pos_x_start + 32;
            let pos_y = (self.line / 8) as u8;

            for pos_x in pos_x_start..pos_x_end {
                let tile_pos = TilePosition::new(pos_x as u8, pos_y);
                let tile = self.build_tile(tile_pos).unwrap();
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

    pub fn build_tile(&self, position: TilePosition) -> Result<Tile> {
        let attributes_id = position.get_attribute_id();

        // ToDo: refactoring here.
        let attribute_addr = attributes_id as u16 + 0x03C0; // 0x03C0 is name table size.
        let attribute = self.vram.read(attribute_addr);

        let palette_id = position.get_palette_id(attribute);
        let palettes = self
            .palette_ram
            .get_palettes(palette_id, PaletteType::Background);
        let sprite_number = self.vram.read(position.get_tile_number() as u16);

        let sprite = self.build_sprite_with_index(*sprite_number)?;

        Ok(Tile {
            sprite,
            position,
            palettes,
        })
    }

    pub fn build_sprite_with_index(&self, index: u8) -> Result<Sprite> {
        let pattern_data = self.pattern_table.get_character_pattern(index as usize)?;
        build_sprite(pattern_data)
    }

    pub fn reset_background(&mut self) {
        self.background = Background::new();
    }

    pub fn read_status(&mut self) -> u8 {
        let status = self.ppu_registers.ppu_status.bits();

        self.ppu_registers.ppu_status.remove(PpuStatus::VBLANK_STARTED);
        self.ppu_registers.ppu_status.remove(PpuStatus::SPRITE_ZERO_HIT);
        self.ppu_registers.ppu_scroll.reset_write_target_is_x();
        self.ppu_registers.ppu_addr.reset_latch();

        status
    }
}

impl PpuRegistration for Ppu {
    fn read(&mut self, address: u16) -> u8 {
        println!("PPU registers read | Address: {:x}", address);
        match address {
            0x0002 => self.read_status(),
            // 0x0004 => {}
            // 0x0007 => {}
            _ => panic!("unimplemented read address: {}", address),
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        println!(
            "PPU registers write | Address: {:x} Data: {:x}",
            address, data
        );
        match address {
            0x0000 => self.ppu_registers.ppu_ctrl = PpuCtrl::from_bits(data).unwrap(),
            0x0001 => self.ppu_registers.ppu_mask = PpuMask::from_bits(data).unwrap(),
            0x0005 => self.ppu_registers.ppu_scroll.write(data),
            0x0006 => self.ppu_registers.ppu_addr.write(data as u16),
            0x0007 => {
                let addr = self.ppu_registers.ppu_addr.read();
                self.ppu_registers.ppu_data.write(
                    addr,
                    data,
                    &mut self.palette_ram,
                    &mut self.vram,
                );
                self.ppu_registers.increment_vram();
            }
            _ => panic!("unimplemented write address: {} / data: {}", address, data),
        }
    }
}

impl<'a> RenderContext<'a> for Ppu {
    fn get_background(&'a self) -> &'a Background {
        &self.background
    }
}

#[cfg(test)]
mod ppu_test {
    use crate::nes::{
        ppu::{palette::PaletteGroup, palette_ram::PaletteRam, sprite::build_sprite, registers::ppu_status::PpuStatus},
        ram::Ram,
    };

    use super::*;

    #[test]
    fn build_sprite_with_index_test() {
        /* build a word 'H'
        Sprite vector.
        3 3 1 0 0 3 3 1
        3 3 1 0 0 3 3 1
        3 3 1 0 0 3 3 1
        3 3 3 3 3 3 3 1
        3 3 1 1 1 3 3 1
        3 3 1 0 0 3 3 1
        3 3 1 0 0 3 3 1
        1 1 1 0 0 1 1 1
        */

        let word_vec = vec![
            // channel 1
            0b11100111, 0b11100111, 0b11100111, 0b11111111, 0b11111111, 0b11100111, 0b11100111,
            0b11100111, // channel 2
            0b11000110, 0b11000110, 0b11000110, 0b11111110, 0b11000110, 0b11000110, 0b11000110,
            0b00000000,
        ];

        let pattern_table = PatternTable::from_vec(word_vec.clone()).unwrap();
        let ppu = Ppu::new(pattern_table, Ram::new(0x4000));
        let result = ppu.build_sprite_with_index(0).unwrap();

        let expect = build_sprite(&word_vec).unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    fn build_sprite_with_index_out_of_pattern_table_range_test() {
        let pattern_table = PatternTable::from_vec(vec![0; 16]).unwrap();
        let ppu = Ppu::new(pattern_table, Ram::new(0x4000));
        let result = ppu.build_sprite_with_index(1);
        assert!(result.is_err());
    }

    #[test]
    fn build_test() {
        /* build a word 'H'
        Sprite vector.
        3 3 1 0 0 3 3 1
        3 3 1 0 0 3 3 1
        3 3 1 0 0 3 3 1
        3 3 3 3 3 3 3 1
        3 3 1 1 1 3 3 1
        3 3 1 0 0 3 3 1
        3 3 1 0 0 3 3 1
        1 1 1 0 0 1 1 1

        Sprite palette
        [0: 15, 1: 0, 2: 16, 3: 32]

        Sprite position
        x: 0, y: 0
        */

        let word_vec = vec![
            // channel 1
            0b11100111, 0b11100111, 0b11100111, 0b11111111, 0b11111111, 0b11100111, 0b11100111,
            0b11100111, // channel 2
            0b11000110, 0b11000110, 0b11000110, 0b11111110, 0b11000110, 0b11000110, 0b11000110,
            0b00000000,
        ];

        // write a palette id: 0 in attribute id: 0
        let mut vram = Ram::new(0x0FFF);
        vram.write(0x03C0, 0x00);

        // write a palettes number in palette id: 0
        let mut palette_ram = PaletteRam::new();
        let palette_numbers = [15, 0, 16, 32];
        palette_ram.write(0x00, palette_numbers[0]);
        palette_ram.write(0x01, palette_numbers[1]);
        palette_ram.write(0x02, palette_numbers[2]);
        palette_ram.write(0x03, palette_numbers[3]);

        let pattern_table = PatternTable::from_vec(word_vec.clone()).unwrap();

        // create ppu context
        let mut ppu = Ppu::new(pattern_table, vram);
        ppu.palette_ram = palette_ram;

        let tile_pos = TilePosition::new(0, 0);
        let tile = ppu.build_tile(tile_pos).unwrap();

        // Assert sprite
        let expect_sprite = build_sprite(&word_vec).unwrap();
        assert_eq!(tile.sprite.to_vec(), expect_sprite.to_vec());

        // assert palettes
        let expect_palettes = PaletteGroup::build(&palette_numbers);
        assert_eq!(tile.palettes, expect_palettes);

        // Assert tile position
        assert_eq!(tile.position.x, 0);
        assert_eq!(tile.position.y, 0);
    }

    #[test]
    fn read_status_test() {
        let dummy_ram1 = Ram::new(0x4000);
        let dummy_ram2 = Ram::new(0x4000);
        let mut ppu = Ppu::new(PatternTable::new(dummy_ram1).unwrap(), dummy_ram2);

        // Setup flag to be cleared
        ppu.ppu_registers.ppu_addr.is_lower_addr = true;
        ppu.ppu_registers.ppu_scroll.write_target_is_x = true;
        ppu.ppu_registers.ppu_status.insert(PpuStatus::VBLANK_STARTED);
        ppu.ppu_registers.ppu_status.insert(PpuStatus::SPRITE_ZERO_HIT);

        let expect_status = ppu.ppu_registers.ppu_status.bits();
        let status = ppu.read_status();

        assert_eq!(status, expect_status);
        assert_eq!(ppu.ppu_registers.ppu_addr.is_lower_addr, false);
        assert_eq!(ppu.ppu_registers.ppu_scroll.write_target_is_x, false);
        assert_eq!(ppu.ppu_registers.ppu_status.contains(PpuStatus::VBLANK_STARTED), false);
        assert_eq!(ppu.ppu_registers.ppu_status.contains(PpuStatus::SPRITE_ZERO_HIT), false);
    }
}
