use crate::config::TILE_SIZE;
use bevy::prelude::*;

use crate::world_map::WorldMap;

pub fn render_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn render_world(mut commands: Commands, world: Res<WorldMap>) {
    for y in 0..world.height {
        for x in 0..world.width {
            let tile = world.get(x, y).unwrap();

            let color = match tile {
                -128..-1 => Color::srgb(0.1, 0.3, 0.8),
                -1..2 => Color::srgb(0.85, 0.75, 0.45),
                2..80 => Color::srgb(0.2, 0.65, 0.2),
                80..90 => Color::srgb(0.05, 0.35, 0.1),
                90..=127 => Color::srgb(0.45, 0.45, 0.45),
            };

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    0.0,
                ),
            ));
        }
    }
}
