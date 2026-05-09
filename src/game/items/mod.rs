pub mod registry;

use bevy::prelude::*;

use self::registry::ItemRegistry;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemRegistry>();
    }
}
