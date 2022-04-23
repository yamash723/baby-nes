use anyhow::{ensure, Result};

use crate::nes::cartridge::flag6::Flag6;

/* Header format
https://www.nesdev.org/wiki/INES
https://archive.nesdev.org/NESHDRJ.TXT

0-3: Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
4: Size of PRG ROM in 16 KB units
5: Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
6: Flags 6 - Mapper, mirroring, battery, trainer
7: Flags 7 - Mapper, VS/Playchoice, NES 2.0
8: Flags 8 - PRG-RAM size (rarely used extension)
9: Flags 9 - TV system (rarely used extension)
10: Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension)
11-15: Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)
*/

/// 4 byte strings, N E S (EOF)
pub const MAGIC_BYTES: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

#[derive(Debug, PartialEq)]
pub struct INesHeader {
    pub magic_bytes: [u8; 4],
    pub program_rom_size: u8,
    pub character_rom_size: u8,
    pub mapper_number: u8,
    pub mirroring: MirroringType,
    pub has_battery_backup: bool,
    pub has_trainer: bool,
    pub four_screen_mode: bool,
}

#[derive(Debug, PartialEq)]
pub enum MirroringType {
    Horizontal,
    Vertical,
}

impl INesHeader {
    pub fn new(binary: &Vec<u8>) -> Result<Self> {
        ensure!(binary.len() >= 7, "binary must be over than 7 bytes");
        ensure!(
            &binary[0..=3] == MAGIC_BYTES.to_vec(),
            "invalid magic bytes"
        );

        let flag6 = Flag6::parse(&binary[6]);

        Ok(Self {
            magic_bytes: MAGIC_BYTES,
            program_rom_size: binary[4],
            character_rom_size: binary[5],
            mapper_number: flag6.lower_mapper_number,
            mirroring: flag6.mirroring,
            has_battery_backup: flag6.has_battery_backup,
            has_trainer: flag6.has_trainer,
            four_screen_mode: flag6.four_screen_mode,
        })
    }
}

#[cfg(test)]
mod ines_header_tests {
    use crate::nes::cartridge::header::{MirroringType, MAGIC_BYTES};
    use std::vec;

    use super::INesHeader;

    #[test]
    fn create_ines_header() {
        let binary = vec![
            MAGIC_BYTES[0],
            MAGIC_BYTES[1],
            MAGIC_BYTES[2],
            MAGIC_BYTES[3],
            3,
            2,
            0b00011010,
        ];

        let header = INesHeader::new(&binary).unwrap();
        assert_eq!(
            header,
            INesHeader {
                magic_bytes: MAGIC_BYTES,
                program_rom_size: 3,
                character_rom_size: 2,
                mapper_number: 1,
                mirroring: MirroringType::Horizontal,
                has_battery_backup: true,
                has_trainer: false,
                four_screen_mode: true,
            }
        );
    }

    #[test]
    fn fails_create_if_less_than_7_bytes() {
        let binary = vec![
            MAGIC_BYTES[0],
            MAGIC_BYTES[1],
            MAGIC_BYTES[2],
            MAGIC_BYTES[3],
            3,
            2,
        ];

        assert!(INesHeader::new(&binary).is_err());
    }

    #[test]
    fn fails_create_if_magic_bytes_not_match() {
        let binary = vec![
            MAGIC_BYTES[0],
            MAGIC_BYTES[1],
            MAGIC_BYTES[2],
            0xFF,
            3,
            2,
            0b00011010,
        ];

        assert!(INesHeader::new(&binary).is_err());
    }
}
