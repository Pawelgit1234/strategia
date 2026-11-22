use bevy::prelude::*;

mod world_map;
mod config;
mod render;
mod generation;
mod controls;
mod settings;

use world_map::WorldMap;
use generation::generate_world;

use crate::{controls::{move_camera, zoom_camera}, render::{render_setup, render_world}, settings::GameSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldMap::new(500, 500))
        .insert_resource(GameSettings::new(rand::random()))
        .add_systems(Startup, (render_setup, generate_world))
        .add_systems(PostStartup, render_world)
        .add_systems(Update, (move_camera, zoom_camera))
        .run();
}
