use super::header::MirroringType;

/* Flag6 format
https://www.nesdev.org/wiki/INES#Flags_6

76543210
||||||||
|||||||+- Mirroring: 0: horizontal (vertical arrangement) (CIRAM A10 = PPU A11)
|||||||              1: vertical (horizontal arrangement) (CIRAM A10 = PPU A10)
||||||+-- 1: Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory
|||||+--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)
||||+---- 1: Ignore mirroring control or above mirroring bit; instead provide four-screen VRAM
++++----- Lower nybble of mapper number
 */
pub struct Flag6 {
    pub lower_mapper_number: u8,
    pub mirroring: MirroringType,
    pub has_battery_backup: bool,
    pub has_trainer: bool,
    pub four_screen_mode: bool,
}

impl Flag6 {
    pub fn parse(byte: &u8) -> Self {
        let mirroring = match byte & 0b00000001 == 0b00000001 {
            false => MirroringType::Horizontal,
            true => MirroringType::Vertical,
        };

        let has_battery_backup = byte & 0b00000010 == 0b00000010;
        let has_trainer = byte & 0b00000100 == 0b00000100;
        let four_screen_mode = byte & 0b00001000 == 0b00001000;

        let lower_mapper_number = byte >> 4;

        Self {
            lower_mapper_number,
            mirroring,
            has_battery_backup,
            has_trainer,
            four_screen_mode,
        }
    }
}

#[cfg(test)]
mod flag6 {
    use crate::nes::cartridge::{flag6::Flag6, header::MirroringType};

    #[test]
    fn bit0_mirroring() {
        let flag6 = Flag6::parse(&0b00000001);
        assert_eq!(flag6.mirroring, MirroringType::Vertical);
    }

    #[test]
    fn bit1_battery_backup() {
        let flag6 = Flag6::parse(&0b00000010);
        assert!(flag6.has_battery_backup);
    }

    #[test]
    fn bit2_trainer() {
        let flag6 = Flag6::parse(&0b00000100);
        assert!(flag6.has_trainer);
    }

    #[test]
    fn bit3_four_screen_mode() {
        let flag6 = Flag6::parse(&0b00001000);
        assert!(flag6.four_screen_mode);
    }

    #[test]
    fn bit4_7_lower_mapper_number() {
        let flag6 = Flag6::parse(&0b10100000);
        assert_eq!(flag6.lower_mapper_number, 0b1010);
    }
}
