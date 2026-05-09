pub mod components;
pub mod systems;

use bevy::prelude::*;

use self::systems::{
    harvest_front_tile_system, place_selected_tile_system, player_movement_system, spawn_player,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                player_movement_system,
                harvest_front_tile_system,
                place_selected_tile_system,
            ),
        );
    }
}
