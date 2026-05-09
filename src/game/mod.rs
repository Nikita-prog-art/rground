use bevy::prelude::*;
use bevy::window::WindowResolution;

pub mod actors;
pub mod agents;
pub mod camera;
pub mod crafting;
pub mod ids;
pub mod inventory;
pub mod items;
pub mod player;
pub mod ui;
pub mod world;

pub const TILE_SIZE: f32 = 32.0;
pub const WORLD_WIDTH: i32 = 112;
pub const WORLD_HEIGHT: i32 = 112;

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.025, 0.027, 0.032)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RGround - sandbox agents prototype".to_string(),
                resolution: WindowResolution::new(1280, 760),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            items::ItemsPlugin,
            world::WorldPlugin,
            inventory::InventoryPlugin,
            crafting::CraftingPlugin,
            player::PlayerPlugin,
            actors::ActorsPlugin,
            agents::AgentPlugin,
            camera::CameraPlugin,
            ui::GameUiPlugin,
        ))
        .run();
}

#[cfg(test)]
mod tests {
    use bevy::input::ButtonInput;
    use bevy::prelude::*;

    use super::*;

    #[test]
    fn game_plugins_initialize_without_query_conflicts() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .add_plugins((
                items::ItemsPlugin,
                world::WorldPlugin,
                inventory::InventoryPlugin,
                crafting::CraftingPlugin,
                player::PlayerPlugin,
                actors::ActorsPlugin,
                agents::AgentPlugin,
                camera::CameraPlugin,
                ui::GameUiPlugin,
            ));

        app.update();
        app.update();
    }
}
