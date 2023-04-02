pub mod ppu_address;
pub mod ppu_control;
pub mod ppu_data;
pub mod ppu_mask;
pub mod ppu_scroll;

use self::{
    ppu_address::PpuAddress,
    ppu_control::{BaseNameTableAddress, PpuCtrl},
    ppu_data::PpuData,
    ppu_mask::PpuMask,
    ppu_scroll::PpuScroll,
};

pub trait PpuRegistration {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
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

    pub fn increment_vram(&mut self) {
        let offset = self.ppu_ctrl.get_vram_increment_offset();
        self.ppu_addr.increment(offset);
    }
}
