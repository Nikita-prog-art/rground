use bevy::prelude::*;

use crate::game::{
    inventory::model::Inventory, items::registry::ItemRegistry, player::components::Player,
    world::map::TileMap,
};

use super::recipes::{RecipeBook, SelectedRecipe, try_craft};

pub fn recipe_selection_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    recipe_book: Res<RecipeBook>,
    mut selected: ResMut<SelectedRecipe>,
) {
    if recipe_book.recipes.is_empty() {
        selected.index = 0;
        return;
    }

    if keyboard.just_pressed(KeyCode::BracketRight) || keyboard.just_pressed(KeyCode::ArrowDown) {
        selected.index = (selected.index + 1) % recipe_book.recipes.len();
        selected.last_status.clear();
    }
    if keyboard.just_pressed(KeyCode::BracketLeft) || keyboard.just_pressed(KeyCode::ArrowUp) {
        selected.index = selected
            .index
            .checked_sub(1)
            .unwrap_or(recipe_book.recipes.len() - 1);
        selected.last_status.clear();
    }
}

pub fn craft_selected_recipe_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    recipe_book: Res<RecipeBook>,
    registry: Res<ItemRegistry>,
    tile_map: Res<TileMap>,
    mut selected: ResMut<SelectedRecipe>,
    mut players: Query<(&Transform, &mut Inventory), With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyC) || recipe_book.recipes.is_empty() {
        return;
    }

    let Some(recipe) = recipe_book.recipes.get(selected.index) else {
        return;
    };

    let Ok((transform, mut inventory)) = players.single_mut() else {
        return;
    };

    let tile_pos = tile_map.world_to_tile(transform.translation);
    if !tile_map.station_available_near(tile_pos, recipe.station) {
        selected.last_status = format!("Need {:?}", recipe.station);
        return;
    }

    match try_craft(&mut inventory, recipe, &registry) {
        Ok(()) => {
            selected.last_status = format!("Crafted {}", registry.name(recipe.output.item));
        }
        Err(error) => {
            selected.last_status = format!("{error:?}");
        }
    }
}
