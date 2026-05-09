pub mod recipes;
pub mod systems;

use bevy::prelude::*;

use self::{
    recipes::{RecipeBook, SelectedRecipe},
    systems::{craft_selected_recipe_system, recipe_selection_system},
};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RecipeBook>()
            .init_resource::<SelectedRecipe>()
            .add_systems(
                Update,
                (recipe_selection_system, craft_selected_recipe_system),
            );
    }
}
