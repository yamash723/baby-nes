use bitflags::bitflags;

bitflags! {
    pub struct PpuMask: u8 {
        const GRAYSCALE                   = 0b00000001; // 0: normal color, 1: produce a grayscale display
        const SHOW_BACKGROUND_IN_LEFTMOST = 0b00000001; // 1: Show background in leftmost 8 pixels of screen, 0: Hide
        const SHOW_SPRITES_IN_LEFTMOST    = 0b00000001; // 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
        const SHOW_BACKGROUND             = 0b00000001; // 1: Show background
        const SHOW_SPRITES                = 0b00000001; // 1: Show sprites
        const EMPHASIZE_RED               = 0b00000001; // Emphasize red*
        const EMPHASIZE_GREEN             = 0b00000001; // Emphasize green*
        const EMPHASIZE_BLUE              = 0b00000001; // Emphasize blue*
      }
}