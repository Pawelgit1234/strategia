use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {
    pub seed: u32
}

impl GameSettings {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }
}
