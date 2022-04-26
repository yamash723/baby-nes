use anyhow::Result;

const SPRITE_WIDTH: usize = 8;
const SPRITE_HEIGHT: usize = 8;

pub type Sprite = Vec<Vec<u8>>;

/* Build sprite from u8 slice.
That slice must be 16 length. (8x16 is not yet supported.)

CHANNEL 1
 0b11111000
 0b11111000
 0b11111000
 0b11111000
 0b11111000             Sprite(1: CHANNEL1 / 2: CHANNEL2 / 3: CHANNEL1 & CHANNEL2)
 0b00000000             [1,1,1,1,1,0,0,0],
 0b00000000 ----┐       [1,1,1,1,1,0,0,0],
 0b00000000     |       [1,1,1,1,1,0,0,0],
                |--->   [1,1,1,3,3,2,2,2],
CHANNEL2        |       [1,1,1,3,3,2,2,2],
 0b00000000     |       [0,0,0,2,2,2,2,2],
 0b00000000 ----┘       [0,0,0,2,2,2,2,2],
 0b00000000             [0,0,0,2,2,2,2,2],
 0b00011111
 0b00011111
 0b00011111
 0b00011111
 0b00011111
*/
pub fn build_sprite(data: &[u8]) -> Result<Sprite> {
    ensure!(data.len() == 16, "invalid length of sprite data.");

    let (channel1, channel2) = data.split_at(8);

    // Overlay two channel data.
    let sprite_data = (0..SPRITE_HEIGHT)
        .map(|y| {
            (1..=SPRITE_WIDTH)
                .map(|x| {
                    let shift_size = SPRITE_WIDTH - x;

                    let ch1_type = channel1[y] >> shift_size & 0b00000001;
                    let ch2_type = (channel2[y] >> shift_size & 0b00000001) * 2;

                    ch1_type + ch2_type
                })
                .collect()
        })
        .collect();

    Ok(sprite_data)
}

#[cfg(test)]
mod sprite_tests {
    use super::build_sprite;

    #[test]
    fn create_sprite() {
        #[rustfmt::skip]
        let channel_1 = vec![
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b00000000,
            0b00000000,
            0b00000000,
        ];

        #[rustfmt::skip]
        let channel_2 = vec![
            0b00000000,
            0b00000000,
            0b00000000,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,
        ];

        let data = [channel_1, channel_2].concat();
        let sprite = build_sprite(&data);

        match sprite {
            Ok(sprite) => assert_eq!(
                sprite,
                vec![
                    vec![1, 1, 1, 1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1, 0, 0, 0],
                    vec![1, 1, 1, 3, 3, 2, 2, 2],
                    vec![1, 1, 1, 3, 3, 2, 2, 2],
                    vec![0, 0, 0, 2, 2, 2, 2, 2],
                    vec![0, 0, 0, 2, 2, 2, 2, 2],
                    vec![0, 0, 0, 2, 2, 2, 2, 2],
                ]
            ),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn sprite_length_must_be_16() {
        let data = vec![0x00; 15];
        let sprite = build_sprite(&data);
        assert!(sprite.is_err());

        let data = vec![0x00; 17];
        let sprite = build_sprite(&data);
        assert!(sprite.is_err());
    }
}
