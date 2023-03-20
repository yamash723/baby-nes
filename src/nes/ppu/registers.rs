pub mod ppu_address;
pub mod ppu_scroll;
pub mod ppu_control;
pub mod ppu_mask;
pub mod ppu_data;

use self::{ppu_scroll::PpuScroll, ppu_address::PpuAddress, ppu_control::{PpuCtrl, BaseNameTableAddress}, ppu_mask::PpuMask, ppu_data::PpuData};

use super::ppu::PpuContext;

pub trait PpuRegistration {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8, ppu_context: &mut PpuContext);
}

pub struct PpuRegisters {
    pub ppu_ctrl: PpuCtrl,
    pub ppu_mask: PpuMask,
    pub ppu_addr: PpuAddress,
    pub ppu_data: PpuData,
    pub ppu_scroll: PpuScroll,
    // pub ppu_status: PpuStatus,
    // pub oam: Oam,
}

impl PpuRegisters {
    pub fn new() -> Self {
        PpuRegisters {
            ppu_ctrl: PpuCtrl::empty(),
            ppu_mask: PpuMask::empty(),
            ppu_addr: PpuAddress::new(),
            ppu_data: PpuData::new(),
            ppu_scroll: PpuScroll::new(),
            // ppu_status: PpuStatus::new(),
            // oam: Oam::new(),
        }
    }

    pub fn get_nametable_address(&self) -> BaseNameTableAddress {
        self.ppu_ctrl.base_name_table_address()
    }

    fn increment_vram(&mut self) {
        let offset = self.ppu_ctrl.get_vram_increment_offset();
        self.ppu_addr.increment(offset);
    }
}

impl PpuRegistration for PpuRegisters {
    fn read(&self, address: u16) -> u8 {
        println!("PPU registers read | Address: {:x}", address);
        unimplemented!();
    }

    fn write(&mut self, address: u16, data: u8, ppu_context: &mut PpuContext) {
        println!("PPU registers write | Address: {:x} Data: {:x}", address, data);
        match address {
            0x0000 => self.ppu_ctrl = PpuCtrl::from_bits(data).unwrap(),
            0x0001 => self.ppu_mask = PpuMask::from_bits(data).unwrap(),
            0x0005 => self.ppu_scroll.write(data),
            0x0006 => self.ppu_addr.write(data as u16),
            0x0007 => {
                let addr = self.ppu_addr.read();
                self.ppu_data.write(addr, data, ppu_context);
                self.increment_vram();
            },
            _ => panic!("unimplemented write address: {} / data: {}", address, data),
        }
    }
}
