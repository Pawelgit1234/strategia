use bevy::{app::prelude, prelude::*};
use noise::{NoiseFn, Perlin};

use crate::world_map::Tile;

use crate::{config::PERLIN_SCALE, world_map::WorldMap};

pub fn generate_world(mut world: ResMut<WorldMap>) {
    let perlin = Perlin::new(rand::random());

    for y in 0..world.height {
        for x in 0..world.width {
            let value = perlin.get([
                x as f64 * PERLIN_SCALE,
                y as f64 * PERLIN_SCALE,
            ]);

            let tile = (value * (Tile::MAX as f64)) as Tile;

            world.set(x, y, tile);
        }
    }
}
