pub mod model;
pub mod systems;

use bevy::prelude::*;

use self::systems::select_hotbar_system;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_hotbar_system);
    }
}
