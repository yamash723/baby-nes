// Must be refactor

use super::tile::Tile;
use std::slice::Iter;

pub struct Background {
    tiles: Vec<Tile>,
}

impl Background {
    pub fn new() -> Self {
        Background { tiles: Vec::new() }
    }

    pub fn push(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn iter(&self) -> Iter<Tile> {
        self.tiles.iter()
    }
}
