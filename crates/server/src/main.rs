use bevy::prelude::*;

mod world_map;
mod config;
mod render;
mod generation;

use world_map::WorldMap;
use generation::generate_world;

use crate::render::{render_setup, render_world};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldMap::new(30, 30))
        .add_systems(Startup, (render_setup, generate_world))
        .add_systems(PostStartup, render_world)
        .run();
}
