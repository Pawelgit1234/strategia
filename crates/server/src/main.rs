use bevy::prelude::*;

mod object_atlas;
mod world_map;
mod config;
mod render;
mod generation;
mod object_components;
mod controls;
mod settings;

use world_map::WorldMap;
use generation::generate_world;

use crate::{controls::{move_camera, zoom_camera}, object_atlas::load_object_atlases, render::{render_objects, render_setup, render_world}, settings::GameSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(AssetPlugin {
                file_path: "../../assets".to_string(),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
         )

        .insert_resource(WorldMap::new(500, 500))
        .insert_resource(GameSettings::new(rand::random()))

        .add_systems(Startup, (load_object_atlases, render_setup, generate_world))

        .add_systems(PostStartup, (render_world, render_objects))
        .add_systems(Update, (move_camera, zoom_camera))
        .run();
}
