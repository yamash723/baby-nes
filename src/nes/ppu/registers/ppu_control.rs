use bitflags::bitflags;

#[derive(Debug, PartialEq)]
pub enum BaseNameTableAddress {
    TYPE0 = 0x2000,
    TYPE1 = 0x2400,
    TYPE2 = 0x2800,
    TYPE3 = 0x2C00,
}

bitflags! {
    pub struct PpuCtrl: u8 {
        const BASE_NAME_TABLE                  = 0b00000011;
        const VRAM_ADDRESS_INCREMENT           = 0b00000100; // 0: add 1, going across; 1: add 32, going down
        const SPRITE_PATTERN_TABLE_ADDRESS     = 0b00001000; // 0: $0000; 1: $1000; ignored in 8x16 mode
        const BACKGROUND_PATTERN_TABLE_ADDRESS = 0b00010000; // 0: $0000; 1: $1000
        const SPRITE_SIZE                      = 0b00100000; // 0: 8x8; 1: 8x16
        const PPU_SELECT_TYPE                  = 0b01000000; // 0: read backdrop from EXT pins; 1: output color on EXT pins
        const READ_BACKDROP_FROM_EXT           = 0b10000000; // 0: off; 1: on
    }
}

impl PpuCtrl {
    pub fn base_name_table_address(&self) -> BaseNameTableAddress {
        match self.bits & 0b00000011 {
            0b00000000 => BaseNameTableAddress::TYPE0,
            0b00000001 => BaseNameTableAddress::TYPE1,
            0b00000010 => BaseNameTableAddress::TYPE2,
            0b00000011 => BaseNameTableAddress::TYPE3,
            _ => unimplemented!(), // Not reachable.
        }
    }

    pub fn get_vram_increment_offset(&self) -> u8 {
        if self.contains(PpuCtrl::VRAM_ADDRESS_INCREMENT) {
            32
        } else {
            1
        }
    }
}

#[cfg(test)]
mod ppu_control_tests {
    use crate::nes::ppu::registers::ppu_control::{PpuCtrl, BaseNameTableAddress};

    #[test]
    fn should_get_base_name_table_address_test() {
        let type0 = PpuCtrl::from_bits(0b00000000).unwrap();
        let type1 = PpuCtrl::from_bits(0b00000001).unwrap();
        let type2 = PpuCtrl::from_bits(0b00000010).unwrap();
        let type3 = PpuCtrl::from_bits(0b00000011).unwrap();

        assert_eq!(type0.base_name_table_address(), BaseNameTableAddress::TYPE0);
        assert_eq!(type1.base_name_table_address(), BaseNameTableAddress::TYPE1);
        assert_eq!(type2.base_name_table_address(), BaseNameTableAddress::TYPE2);
        assert_eq!(type3.base_name_table_address(), BaseNameTableAddress::TYPE3);
    }
}