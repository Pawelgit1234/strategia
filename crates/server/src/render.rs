use crate::{config::{BRIGHTNESS_FACTOR, PERLIN_SCALE, TILE_SIZE}, settings::GameSettings};
use bevy::prelude::*;
use noise::{NoiseFn, Worley};

use crate::world_map::WorldMap;

pub fn render_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn worley_color(overlay: &Worley, color: u32, x: usize, y: usize) -> Color {
    let value = overlay.get([
        x as f64 * PERLIN_SCALE,
        y as f64 * PERLIN_SCALE,
    ]) as f32;

    let brightness = (1.0 + value * BRIGHTNESS_FACTOR)
        .clamp(0.5, 1.5);

    let mut hsva = Hsva::from(Color::srgb_u32(color));

    hsva.value = (hsva.value * brightness).clamp(0.0, 1.0);

    Color::from(hsva)
}

pub fn render_world(
    mut commands: Commands,
    world: Res<WorldMap>,
    settings: Res<GameSettings>,
) {
    let overlay = Worley::new(settings.seed);

    for y in 0..world.height {
        for x in 0..world.width {
            let Some(tile) = world.get(x, y) else {
                continue;
            };

            let base_color = match tile {
                -128..-70 => 0x123C65,
                -70..-35  => 0x1B6385,
                -35..-10  => 0x2485A6,
                -10..-5   => 0xE3C054,
                -5..20    => 0xA1BF41,
                20..40    => 0x55A058,
                40..55    => 0x1E7453,
                55..70    => 0x878573,
                70..90    => 0x69645F,
                90..=127  => 0xF5F3E8,
            };

            let color = worley_color(&overlay, base_color, x, y);

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
