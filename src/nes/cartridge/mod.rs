mod flag6;
pub mod header;

use anyhow::Result;
use header::INesHeader;

const PROGRAM_UNIT_SIZE: usize = 16384; // 16384 byte
const CHARACTER_UNIT_SIZE: usize = 8192; // 8192 byte

#[derive(Debug, PartialEq)]
pub struct Cartridge {
    pub header: INesHeader,
    pub program_rom: Vec<u8>,
    pub character_rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(binary: &[u8]) -> Result<Self> {
        ensure!(binary.len() >= 16, "binary must be over than 16 bytes");

        let header_binary = binary[0..16].to_vec();
        let header = match INesHeader::new(&header_binary) {
            Ok(header) => header,
            _ => bail!("invalid header binary"),
        };

        let program_rom_byte_size = PROGRAM_UNIT_SIZE * header.program_rom_size as usize;
        let character_rom_byte_size = CHARACTER_UNIT_SIZE * header.character_rom_size as usize;

        let rom_binary = binary[16..].to_vec();
        if rom_binary.len() != program_rom_byte_size + character_rom_byte_size {
            bail!("invalid PRG-ROM or CHR-ROM binary size.");
        }

        let (program_rom, character_rom) = rom_binary.split_at(program_rom_byte_size);

        Ok(Cartridge {
            header,
            program_rom: program_rom.to_vec(),
            character_rom: character_rom.to_vec(),
        })
    }
}

#[cfg(test)]
mod cartridge_tests {
    use crate::nes::cartridge::{CHARACTER_UNIT_SIZE, PROGRAM_UNIT_SIZE};
    use std::vec;

    use super::{header::MAGIC_BYTES, Cartridge};

    fn build_correct_binary() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let program_rom_size: u8 = 2;
        let character_rom_size: u8 = 1;
        let flag6 = 0b00011010;

        let program_rom = vec![0x10u8; PROGRAM_UNIT_SIZE * program_rom_size as usize];
        let character_rom = vec![0x20u8; CHARACTER_UNIT_SIZE * character_rom_size as usize];

        let header_binary = [
            vec![
                MAGIC_BYTES[0],
                MAGIC_BYTES[1],
                MAGIC_BYTES[2],
                MAGIC_BYTES[3],
                program_rom_size,
                character_rom_size,
                flag6,
            ],
            // padding. header must be 16bytes.
            vec![0; 9],
        ]
        .concat();

        (header_binary, program_rom, character_rom)
    }

    #[test]
    fn create_cartridge() {
        let (header_binary, program_rom, character_rom) = build_correct_binary();

        let binary = [header_binary, program_rom.clone(), character_rom.clone()].concat();
        let cartridge = Cartridge::new(&binary).unwrap();

        assert_eq!(cartridge.program_rom, program_rom);
        assert_eq!(cartridge.character_rom, character_rom);
    }

    #[test]
    fn fails_create_if_binary_data_length_insufficient() {
        let (header_binary, program_rom, character_rom) = build_correct_binary();
        let mut binary = [header_binary, program_rom.clone(), character_rom.clone()].concat();

        binary.pop();

        let cartridge = Cartridge::new(&binary);

        assert!(cartridge.is_err());
    }
}
