pub(super) mod arithmetic;
pub(super) mod branch;
pub(super) mod decrement;
pub(super) mod flags;
pub(super) mod increment;
pub(super) mod jump;
pub(super) mod load;
pub(super) mod logical;
pub(super) mod shift;
pub(super) mod stack;
pub(super) mod store;
pub(super) mod system;
pub(super) mod transfer;

#[cfg(test)]
mod instructions_test {
    use crate::nes::bus::Bus;

    pub struct MockBus {
        data: Vec<u8>,
    }

    impl MockBus {
        pub fn new() -> Self {
            Self {
                data: vec![0; 0x10000],
            }
        }
    }

    impl Bus for MockBus {
        fn read(&self, address: u16) -> u8 {
            self.data[address as usize]
        }

        fn read_u16(&self, address: u16) -> u16 {
            let lower = self.data[address as usize];
            let upper = self.data[(address + 1) as usize];
            u16::from_be_bytes([upper, lower])
        }

        fn write(&mut self, address: u16, data: u8) {
            self.data[address as usize] = data;
        }
    }
}
