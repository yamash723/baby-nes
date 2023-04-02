pub struct PpuAddress {
    addr: u16,
    is_lower_addr: bool,
}

impl PpuAddress {
    pub fn new() -> Self {
        PpuAddress {
            addr: 0,
            is_lower_addr: false,
        }
    }

    pub fn read(&self) -> u16 {
        self.addr
    }

    pub fn increment(&mut self, offset: u8) {
        self.addr += offset as u16;
    }

    pub fn write(&mut self, data: u16) {
        if self.is_lower_addr {
            self.addr += data;
        } else {
            self.addr = (data) << 8;
        }

        self.is_lower_addr = !self.is_lower_addr;
    }

    pub fn reset_latch(&mut self) {
        self.is_lower_addr = false;
    }
}

#[cfg(test)]
mod ppu_addr_test {
    use super::*;

    #[test]
    fn write_u8_twice_to_make_u16() {
        let mut ppu_addr = PpuAddress::new();
        let upper_addr = 0x23;
        let lower_addr = 0x45;

        ppu_addr.write(upper_addr);
        assert_eq!(ppu_addr.addr, 0x2300);

        ppu_addr.write(lower_addr);
        assert_eq!(ppu_addr.addr, 0x2345);
    }

    #[test]
    fn reset_latch_test() {
        let mut ppu_addr = PpuAddress::new();
        ppu_addr.is_lower_addr = true;

        ppu_addr.reset_latch();
        assert_eq!(ppu_addr.is_lower_addr, false);
    }
}
