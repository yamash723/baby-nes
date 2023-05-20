use bitflags::bitflags;

bitflags! {
    pub struct PpuStatus: u8 {
        const VBLANK_STARTED  = 0b00000001; // 0: not in vblank; 1: in vblank
        const SPRITE_ZERO_HIT = 0b00000010; // 0: no sprite 0 hit; 1: sprite 0 hit
        const SPRITE_OVERFLOW = 0b00000100; // 0: no sprite overflow; 1: sprite overflow
        // const PPU_OPEN_BUS    = 0b11111000; // unused bits
      }
}





