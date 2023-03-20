use std::ops::Range;

pub struct Ram {
    data: Vec<u8>,
}

impl <'a> Ram {
    pub fn new(size: u16) -> Self {
        Self { data: vec!(0; size.into()) }
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn read(&'a self, address: u16) -> &'a u8 {
        match self.data.get(address as usize) {
            Some(data) => data,
            None => panic!("Out-of-range access to RAM. RAM size {:X} / address: {:X}", self.data.len(), address),
        }
    }

    pub fn read_range(&'a self, range: Range<usize>) -> &'a [u8] {
        match self.data.get(range.clone()) {
            Some(data) => data,
            None => panic!("Out-of-range access to RAM. RAM size {:X} / range: {:?}", self.data.len(), range),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match self.data.get_mut(address as usize) {
            Some(data) => *data = value,
            None => panic!("Out-of-range access to RAM. RAM size {:X} / address: {:X}", self.data.len(), address),
        }
    }
}


#[cfg(test)]
mod ram_tests {
    use super::Ram;

    #[test]
    fn read_test() {
        let ram = Ram::new(10);
        assert_eq!(ram.read(0x009), &0x00);
    }

    #[test]
    #[should_panic]
    fn outrange_read_should_panic_test() {
        let ram = Ram::new(1);
        ram.read(0x001);
    }

    #[test]
    fn read_range_test() {
        let ram = Ram::new(10);
        assert_eq!(ram.read_range(0..2), &[0x00, 0x00]);
    }

    #[test]
    #[should_panic]
    fn outrange_read_range_should_panic_test() {
        let ram = Ram::new(1);
        ram.read_range(1..2);
    }

    #[test]
    fn write_test() {
        let mut ram = Ram::new(10);
        ram.write(0x001, 0x20);

        assert_eq!(ram.read(0x001), &0x20);
    }

    #[test]
    #[should_panic]
    fn outrange_write_should_panic_test() {
        let mut ram = Ram::new(1);
        ram.write(0x001, 0x20);
    }
}
