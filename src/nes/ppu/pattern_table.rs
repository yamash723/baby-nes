use anyhow::Result;

use crate::nes::ram::Ram;

pub struct PatternTable(Ram);

impl <'a> PatternTable {
    pub fn new(data: Ram) -> Result<Self> {
        if data.len() % 16 != 0 {
            return Err(anyhow!("PatternTable data must be multiple of 16."));
        }

        Ok(PatternTable(data))
    }

    pub fn from_vec(data: Vec<u8>) -> Result<Self> {
        Self::new(Ram::from_vec(data))
    }

    pub fn read(&'a self, addr: u16) -> &'a u8 {
        self.0.read(addr)
    }

    pub fn get_character_pattern(&'a self, index: usize) -> Result<&'a [u8]> {
        if self.0.len() < (index + 1) * 16 {
            return Err(anyhow!("Out-of-range access to PatternTable. RAM size {} / index: {}", self.0.len() / 16, index));
        }

        let start = index * 16;
        let end = start + 16;

        Ok(&self.0.read_range(start..end))
    }
}

#[cfg(test)]
mod pattern_table_test {
    use super::*;

    #[test]
    fn pattern_table_data_must_be_multiple_of_16_test() {
        let valid_pattern_table = PatternTable::new(Ram::from_vec(vec![0; 32]));
        assert!(valid_pattern_table.is_ok());
        assert_eq!(valid_pattern_table.unwrap().0.len(), 32);

        let invalid_pattern_table = PatternTable::new(Ram::from_vec(vec![0; 33]));
        assert!(invalid_pattern_table.is_err());
    }

    #[test]
    fn get_character_pattern_test() {
        let pattern_data = vec![
            // channel 1
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b00000000,
            0b00000000,
            0b00000000,
            // channel 2
            0b00000000,
            0b00000000,
            0b00000000,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,
        ];

        let mut character_rom = vec![0b00000000; 16]; // Offset data.
        character_rom.extend_from_slice(&pattern_data);

        let pattern_table = PatternTable::from_vec(character_rom).unwrap();

        let result = pattern_table.get_character_pattern(1).unwrap();
        assert_eq!(result, &pattern_data);
    }

    #[test]
    fn should_be_return_error_to_out_of_range_access_test() {
        let pattern_table = PatternTable::from_vec(Vec::new()).unwrap();
        assert!(pattern_table.get_character_pattern(0).is_err());
    }
}
