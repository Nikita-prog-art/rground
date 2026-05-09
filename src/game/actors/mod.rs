pub mod components;
pub mod spawn;
pub mod systems;

use bevy::prelude::*;

use self::{
    spawn::spawn_npcs_and_mobs,
    systems::{mob_ai_system, mob_motion_system},
};

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<systems::MobAiClock>()
            .add_systems(Startup, spawn_npcs_and_mobs)
            .add_systems(Update, (mob_ai_system, mob_motion_system));
    }
}
