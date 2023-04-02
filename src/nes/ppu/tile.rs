use super::palette::PaletteGroup;
use super::sprite::Sprite;
use super::tile_position::TilePosition;

#[derive(Debug)]
pub struct Tile {
    pub sprite: Sprite,
    pub position: TilePosition,
    pub palettes: PaletteGroup,
}
