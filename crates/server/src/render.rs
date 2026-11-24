use crate::{config::{BRIGHTNESS_FACTOR, PERLIN_SCALE, TILE_SIZE}, object_atlas::ObjectAtlases, object_components::{Position, WorldObject}, settings::GameSettings};
use bevy::prelude::*;
use noise::{NoiseFn, Worley};

use crate::world_map::WorldMap;


// TODO: trees and bushes on ferile land
// TODO: debug text on side in monocraft
// TODO: optimize => to much entities

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
                -128..-70 => 0x123C65, // Deep water
                -70..-35  => 0x1B6385, // Medium water
                -35..-10  => 0x2485A6, // Shallow water

                -10..-5   => 0xE3C054, // Sand

                -5..20    => 0xA1BF41, // Soil
                20..40    => 0x55A058, // Fertile soil
                40..55    => 0x1E7453, // Forest
                //
                55..70    => 0x878573, // Rock
                70..90    => 0x69645F, // Mountain
                90..=127  => 0xF5F3E8, // Snow peak
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

pub fn render_objects(
    mut commands: Commands,
    query: Query<(&Position, &WorldObject)>,
    atlases: Res<ObjectAtlases>,
) {

    for (position, object) in &query {
        let (image, layout, count) = match object {
            WorldObject::Tree => (
                atlases.trees_image.clone(),
                atlases.trees_layout.clone(),
                16,
            ),

            WorldObject::Bush => (
                atlases.bushes_image.clone(),
                atlases.bushes_layout.clone(),
                16,
            ),
        };

        let index = rand::random_range(0..count);

        commands.spawn((
            Sprite {
                image,
                texture_atlas: Some(TextureAtlas {
                    layout,
                    index,
                }),
                ..default()
            },
            Transform::from_xyz(
                position.x as f32 * TILE_SIZE,
                position.y as f32 * TILE_SIZE,
                position.y as f32 * 0.001,
            ),
        ));
    }
}
