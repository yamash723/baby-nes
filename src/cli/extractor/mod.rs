mod sprite_writer;

use std::fmt::Result;

use crate::nes::{
    cartridge::Cartridge,
    ppu::sprite::{build_sprite, Sprite},
};

use self::sprite_writer::SpriteImageWriter;

pub struct SpriteExtractor<'a> {
    cartridge: &'a Cartridge,
}

impl<'a> SpriteExtractor<'a> {
    pub fn new(cartridge: &'a Cartridge) -> Self {
        Self { cartridge }
    }

    pub fn extract_sprite(self, path: &str) -> Result {
        // ToDo
        let sprite_length = self.cartridge.character_rom.len() / 16;

        let sprites: Vec<Sprite> = (0..sprite_length)
            .map(|i| {
                let start = i * 16;
                let end = start + 16;
                let data = &self.cartridge.character_rom[start..end];

                build_sprite(data).unwrap()
            })
            .collect();

        let writer = SpriteImageWriter::new(&sprites, 50);
        writer.save(path);

        Ok(())
    }
}
