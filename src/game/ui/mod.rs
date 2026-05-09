pub mod hud;

use bevy::input_focus::InputFocus;
use bevy::prelude::*;

use self::hud::{
    button_interaction_system, setup_game_ui, toggle_inventory_panel_system,
    update_agent_stats_system, update_hotbar_system, update_inventory_panel_system,
    update_player_hud_system,
};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>()
            .init_resource::<hud::InventoryPanelState>()
            .add_systems(Startup, setup_game_ui)
            .add_systems(
                Update,
                (
                    toggle_inventory_panel_system,
                    button_interaction_system,
                    update_player_hud_system,
                    update_hotbar_system,
                    update_inventory_panel_system,
                    update_agent_stats_system,
                ),
            );
    }
}
