#[macro_use]
extern crate anyhow;

use std::{fs::File, io::Read};

use crate::{cli::extractor::SpriteExtractor, nes::cartridge::Cartridge};

pub mod cli;
pub mod nes;

fn main() {
    let mut f = File::open("rom/hello_world.nes").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    let cartridge = Cartridge::new(&buffer).unwrap();
    let extractor = SpriteExtractor::new(&cartridge);

    extractor.extract_sprite("sprite.png").unwrap();
}
