use bitflags::bitflags;

bitflags! {
    pub struct CpuStatusFlag: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL           = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
    pub p: CpuStatusFlag,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            p: CpuStatusFlag { bits: 0x00000000 },
            pc: 0,
        }
    }

    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.p.insert(CpuStatusFlag::ZERO);
        } else {
            self.p.remove(CpuStatusFlag::ZERO);
        }

        if result >> 7 == 1 {
            self.p.insert(CpuStatusFlag::NEGATIVE);
        } else {
            self.p.remove(CpuStatusFlag::NEGATIVE);
        }
    }
}
