pub mod generation;
pub mod map;
pub mod systems;
pub mod tiles;

use bevy::prelude::*;

use crate::game::{WORLD_HEIGHT, WORLD_WIDTH};

use self::{generation::generate_world, systems::spawn_world_tiles};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(generate_world(WORLD_WIDTH, WORLD_HEIGHT))
            .init_resource::<map::TileVisuals>()
            .add_systems(Startup, spawn_world_tiles);
    }
}
