use super::{background::Background, frame::Frame};

pub trait RenderContext<'a> {
    fn get_background(&'a self) -> &'a Background;
}

pub fn rendering_frame<'a, T>(ctx: &'a T) -> Frame
where
    T: RenderContext<'a>,
{
    let background = ctx.get_background();
    let mut frame = Frame::new();

    for tile in background.iter() {
        let sprite = &tile.sprite;
        let position = &tile.position;

        /*
          The background line consists of 32 tiles.
          One line consists of 256px and One tile consists of 8px.
          +----+----+----+----+----+----+----+----+----+---
          |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |  8 | ..
          +----+----+----+----+----+----+----+----+----+---
          | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | ..
          +----+----+----+----+----+----+----+----+----+---

          ex: Starting position for drawing if tile position No. 2
          x = 8 x 2 = 16
          y = 8 x 0 = 0
        */
        let start_pos_x = position.x as usize * 8;
        let start_pos_y = position.y as usize * 8;

        for (y, sprite_line) in sprite.to_vec().iter().enumerate() {
            for (x, palette_number) in sprite_line.iter().enumerate() {
                let point_x = start_pos_x + x;
                let point_y = start_pos_y + y;
                let palette = tile.palettes.get(*palette_number as usize);
                let color = palette.get_color_code();

                frame.set_pixel(point_x, point_y, color);
            }
        }
    }

    frame
}

#[cfg(test)]
mod render_tests {
    use crate::nes::ppu::{
        background::Background,
        palette::{PaletteGroup, NES_COLORS},
        sprite::build_sprite,
        tile::Tile,
        tile_position::TilePosition,
    };

    use super::{rendering_frame, RenderContext};
    struct TestRenderContext {
        data: Background,
    }

    impl<'a> RenderContext<'a> for TestRenderContext {
        fn get_background(&'a self) -> &'a Background {
            &self.data
        }
    }

    #[test]
    fn rendering_frame_test() {
        #[rustfmt::skip]
        let pattern = vec![
            // channel 1
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b11111000,
            0b00000000,
            0b00000000,
            0b00000000,
            // channel 2
            0b00000000,
            0b00000000,
            0b00000000,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,
            0b00011111,
        ];
        let sprite = build_sprite(&pattern).unwrap();

        let (pos_x, pos_y) = (2, 3);
        let position = TilePosition::new(pos_x, pos_y);

        let palettes = PaletteGroup::build(&[2, 4, 5, 6]);
        let tile = Tile {
            sprite,
            position,
            palettes,
        };

        let mut background = Background::new();
        background.push(tile);

        // Rendering
        let ctx = TestRenderContext { data: background };
        let frame = rendering_frame(&ctx);

        let c0 = NES_COLORS[2];
        let c1 = NES_COLORS[4];
        let c2 = NES_COLORS[5];
        let c3 = NES_COLORS[6];

        let expect_rendered_tile = vec![
            vec![c1, c1, c1, c1, c1, c0, c0, c0],
            vec![c1, c1, c1, c1, c1, c0, c0, c0],
            vec![c1, c1, c1, c1, c1, c0, c0, c0],
            vec![c1, c1, c1, c3, c3, c2, c2, c2],
            vec![c1, c1, c1, c3, c3, c2, c2, c2],
            vec![c0, c0, c0, c2, c2, c2, c2, c2],
            vec![c0, c0, c0, c2, c2, c2, c2, c2],
            vec![c0, c0, c0, c2, c2, c2, c2, c2],
        ];

        let start_pos_x = (pos_x * 8) as usize;
        let start_pos_y = (pos_y * 8) as usize;

        for x in 0..8 as usize {
            for y in 0..8 as usize {
                let point_x = start_pos_x + x;
                let point_y = start_pos_y + y;

                let pixel = frame.get_pixel(point_x, point_y);
                let color = expect_rendered_tile[y][x];

                assert_eq!(pixel, (color[0], color[1], color[2]));
            }
        }
    }
}
