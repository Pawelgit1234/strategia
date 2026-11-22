use bevy::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

use crate::settings::GameSettings;
use crate::world_map::Tile;

use crate::{config::PERLIN_SCALE, world_map::WorldMap};

pub fn generate_world(mut world: ResMut<WorldMap>, settings: Res<GameSettings>) {
    let perlin = Fbm::<Perlin>::new(settings.seed)
        .set_octaves(6)
        .set_frequency(0.015)
        .set_lacunarity(2.0)
        .set_persistence(0.5);

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
