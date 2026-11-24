use bevy::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

use crate::config::OBJECT_PERLIN_LIMIT;
use crate::object_components::{Position, WorldObject};
use crate::settings::GameSettings;
use crate::world_map::Tile;

use crate::{config::PERLIN_SCALE, world_map::WorldMap};

pub fn generate_world(
    mut world: ResMut<WorldMap>,
    mut commands: Commands,
    settings: Res<GameSettings>,
) {
    let terrain_noise = Fbm::<Perlin>::new(settings.seed)
        .set_octaves(6)
        .set_frequency(0.015)
        .set_lacunarity(2.0)
        .set_persistence(0.5);

    let object_noise = Perlin::new(settings.seed + 1);

    for y in 0..world.height {
        for x in 0..world.width {
            let coords = [x as f64 * PERLIN_SCALE, y as f64 * PERLIN_SCALE];

            let value = terrain_noise.get(coords);
            let tile = (value * Tile::MAX as f64) as Tile;

            let object = match tile {
                20..40 => Some(WorldObject::Tree),
                40..55 => Some(WorldObject::Bush),
                _ => None,
            };

            if let Some(object) = object {
                let object_value = object_noise.get(coords);

                if object_value > OBJECT_PERLIN_LIMIT {
                    commands.spawn((Position { x, y }, object));
                }
            }

            world.set(x, y, tile);
        }
    }
}
