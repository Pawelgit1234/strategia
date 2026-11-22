use bevy::prelude::*;

pub type Tile = i8;

#[derive(Resource)]
pub struct WorldMap {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Tile>
}

impl WorldMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![0; width * height]
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.tiles[y * self.width + x])
    }

    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if x < self.width && y < self.height {
            self.tiles[y * self.width + x] = tile;
        }
    }
}
