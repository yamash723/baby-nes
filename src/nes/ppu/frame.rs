pub struct Frame {
  /* Frame has WIDTH x HEIGHT pixels.
  pixel is represented by 3 bytes (RGB).

  ex) 4px x 4px
  [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0],
  [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0],
  [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0],
  [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0],
  */
  pub data: Vec<u8>,
}

impl Frame {
  pub const WIDTH: usize = 256;
  pub const HIGHT: usize = 240;

  pub fn new() -> Self {
    Frame {
      data: vec![0; (Frame::WIDTH) * (Frame::HIGHT) * 3],
    }
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
    let base = y * 3 * Frame::WIDTH + x * 3;
    if base + 2 < self.data.len() {
      self.data[base] = rgb.0;
      self.data[base + 1] = rgb.1;
      self.data[base + 2] = rgb.2;
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_pixel() {
        let mut frame = Frame::new();
        frame.set_pixel(0, 0, (11, 22, 33));
        assert_eq!(frame.data[0], 11);
        assert_eq!(frame.data[1], 22);
        assert_eq!(frame.data[2], 33);

        frame.set_pixel(10, 20, (255, 0, 0));
        assert_eq!(frame.data[20 * 3 * Frame::WIDTH + 10 * 3], 255);
        assert_eq!(frame.data[20 * 3 * Frame::WIDTH + 10 * 3 + 1], 0);
        assert_eq!(frame.data[20 * 3 * Frame::WIDTH + 10 * 3 + 2], 0);
    }
}
