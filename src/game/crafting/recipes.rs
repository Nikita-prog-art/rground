use bevy::prelude::*;

use crate::game::{ids::ItemId, inventory::model::Inventory, items::registry::ItemRegistry};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CraftStation {
    Hand,
    Workbench,
    Furnace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecipeCategory {
    Blocks,
    Tools,
    Combat,
    Food,
    Utility,
    Smelting,
}

#[derive(Clone, Debug)]
pub struct Stack {
    pub item: ItemId,
    pub quantity: u32,
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub id: &'static str,
    pub name: &'static str,
    pub output: Stack,
    pub ingredients: Vec<Stack>,
    pub station: CraftStation,
    pub category: RecipeCategory,
}

#[derive(Resource)]
pub struct RecipeBook {
    pub recipes: Vec<Recipe>,
}

#[derive(Resource, Default)]
pub struct SelectedRecipe {
    pub index: usize,
    pub last_status: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CraftError {
    MissingIngredient,
    NoSpace,
}

impl Default for RecipeBook {
    fn default() -> Self {
        Self {
            recipes: minecraft_like_recipes(),
        }
    }
}

pub fn can_craft(inventory: &Inventory, recipe: &Recipe, registry: &ItemRegistry) -> bool {
    recipe
        .ingredients
        .iter()
        .all(|ingredient| inventory.has(ingredient.item, ingredient.quantity))
        && inventory.can_add(recipe.output.item, recipe.output.quantity, registry)
}

pub fn try_craft(
    inventory: &mut Inventory,
    recipe: &Recipe,
    registry: &ItemRegistry,
) -> Result<(), CraftError> {
    if !recipe
        .ingredients
        .iter()
        .all(|ingredient| inventory.has(ingredient.item, ingredient.quantity))
    {
        return Err(CraftError::MissingIngredient);
    }

    if !inventory.can_add(recipe.output.item, recipe.output.quantity, registry) {
        return Err(CraftError::NoSpace);
    }

    for ingredient in &recipe.ingredients {
        inventory.remove(ingredient.item, ingredient.quantity);
    }
    inventory.add(recipe.output.item, recipe.output.quantity, registry);

    Ok(())
}

fn stack(item: ItemId, quantity: u32) -> Stack {
    Stack { item, quantity }
}

fn recipe(
    id: &'static str,
    name: &'static str,
    output: Stack,
    ingredients: Vec<Stack>,
    station: CraftStation,
    category: RecipeCategory,
) -> Recipe {
    Recipe {
        id,
        name,
        output,
        ingredients,
        station,
        category,
    }
}

fn minecraft_like_recipes() -> Vec<Recipe> {
    use CraftStation::*;
    use RecipeCategory::*;

    vec![
        recipe(
            "oak_planks",
            "Oak Planks",
            stack("oak_planks", 4),
            vec![stack("oak_log", 1)],
            Hand,
            Blocks,
        ),
        recipe(
            "birch_planks",
            "Birch Planks",
            stack("birch_planks", 4),
            vec![stack("birch_log", 1)],
            Hand,
            Blocks,
        ),
        recipe(
            "spruce_planks",
            "Spruce Planks",
            stack("spruce_planks", 4),
            vec![stack("spruce_log", 1)],
            Hand,
            Blocks,
        ),
        recipe(
            "sticks_oak",
            "Sticks",
            stack("stick", 4),
            vec![stack("oak_planks", 2)],
            Hand,
            Utility,
        ),
        recipe(
            "sticks_birch",
            "Sticks",
            stack("stick", 4),
            vec![stack("birch_planks", 2)],
            Hand,
            Utility,
        ),
        recipe(
            "crafting_table",
            "Crafting Table",
            stack("crafting_table", 1),
            vec![stack("oak_planks", 4)],
            Hand,
            Utility,
        ),
        recipe(
            "torch",
            "Torches",
            stack("torch", 4),
            vec![stack("coal", 1), stack("stick", 1)],
            Hand,
            Utility,
        ),
        recipe(
            "chest",
            "Chest",
            stack("chest", 1),
            vec![stack("oak_planks", 8)],
            Workbench,
            Utility,
        ),
        recipe(
            "furnace",
            "Furnace",
            stack("furnace", 1),
            vec![stack("cobblestone", 8)],
            Workbench,
            Utility,
        ),
        recipe(
            "ladder",
            "Ladder",
            stack("ladder", 3),
            vec![stack("stick", 7)],
            Workbench,
            Utility,
        ),
        recipe(
            "oak_door",
            "Oak Door",
            stack("oak_door", 3),
            vec![stack("oak_planks", 6)],
            Workbench,
            Utility,
        ),
        recipe(
            "oak_fence",
            "Oak Fence",
            stack("oak_fence", 3),
            vec![stack("oak_planks", 4), stack("stick", 2)],
            Workbench,
            Blocks,
        ),
        recipe(
            "paper",
            "Paper",
            stack("paper", 3),
            vec![stack("wheat", 3)],
            Workbench,
            Utility,
        ),
        recipe(
            "book",
            "Book",
            stack("book", 1),
            vec![stack("paper", 3), stack("leather", 1)],
            Workbench,
            Utility,
        ),
        recipe(
            "bed",
            "Bed",
            stack("bed", 1),
            vec![stack("white_wool", 3), stack("oak_planks", 3)],
            Workbench,
            Utility,
        ),
        recipe(
            "bread",
            "Bread",
            stack("bread", 1),
            vec![stack("wheat", 3)],
            Hand,
            Food,
        ),
        recipe(
            "arrow",
            "Arrows",
            stack("arrow", 4),
            vec![stack("flint", 1), stack("stick", 1), stack("feather", 1)],
            Workbench,
            Combat,
        ),
        recipe(
            "bow",
            "Bow",
            stack("bow", 1),
            vec![stack("stick", 3), stack("string", 3)],
            Workbench,
            Combat,
        ),
        recipe(
            "shield",
            "Shield",
            stack("shield", 1),
            vec![stack("oak_planks", 6), stack("iron_ingot", 1)],
            Workbench,
            Combat,
        ),
        recipe(
            "bucket",
            "Bucket",
            stack("bucket", 1),
            vec![stack("iron_ingot", 3)],
            Workbench,
            Utility,
        ),
        recipe(
            "wooden_sword",
            "Wooden Sword",
            stack("wooden_sword", 1),
            vec![stack("oak_planks", 2), stack("stick", 1)],
            Workbench,
            Combat,
        ),
        recipe(
            "stone_sword",
            "Stone Sword",
            stack("stone_sword", 1),
            vec![stack("cobblestone", 2), stack("stick", 1)],
            Workbench,
            Combat,
        ),
        recipe(
            "iron_sword",
            "Iron Sword",
            stack("iron_sword", 1),
            vec![stack("iron_ingot", 2), stack("stick", 1)],
            Workbench,
            Combat,
        ),
        recipe(
            "diamond_sword",
            "Diamond Sword",
            stack("diamond_sword", 1),
            vec![stack("diamond", 2), stack("stick", 1)],
            Workbench,
            Combat,
        ),
        recipe(
            "wooden_pickaxe",
            "Wooden Pickaxe",
            stack("wooden_pickaxe", 1),
            vec![stack("oak_planks", 3), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "stone_pickaxe",
            "Stone Pickaxe",
            stack("stone_pickaxe", 1),
            vec![stack("cobblestone", 3), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "iron_pickaxe",
            "Iron Pickaxe",
            stack("iron_pickaxe", 1),
            vec![stack("iron_ingot", 3), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "diamond_pickaxe",
            "Diamond Pickaxe",
            stack("diamond_pickaxe", 1),
            vec![stack("diamond", 3), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "wooden_axe",
            "Wooden Axe",
            stack("wooden_axe", 1),
            vec![stack("oak_planks", 3), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "stone_axe",
            "Stone Axe",
            stack("stone_axe", 1),
            vec![stack("cobblestone", 3), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "iron_axe",
            "Iron Axe",
            stack("iron_axe", 1),
            vec![stack("iron_ingot", 3), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "diamond_axe",
            "Diamond Axe",
            stack("diamond_axe", 1),
            vec![stack("diamond", 3), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "wooden_shovel",
            "Wooden Shovel",
            stack("wooden_shovel", 1),
            vec![stack("oak_planks", 1), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "stone_shovel",
            "Stone Shovel",
            stack("stone_shovel", 1),
            vec![stack("cobblestone", 1), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "iron_shovel",
            "Iron Shovel",
            stack("iron_shovel", 1),
            vec![stack("iron_ingot", 1), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "diamond_shovel",
            "Diamond Shovel",
            stack("diamond_shovel", 1),
            vec![stack("diamond", 1), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "wooden_hoe",
            "Wooden Hoe",
            stack("wooden_hoe", 1),
            vec![stack("oak_planks", 2), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "stone_hoe",
            "Stone Hoe",
            stack("stone_hoe", 1),
            vec![stack("cobblestone", 2), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "iron_hoe",
            "Iron Hoe",
            stack("iron_hoe", 1),
            vec![stack("iron_ingot", 2), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "diamond_hoe",
            "Diamond Hoe",
            stack("diamond_hoe", 1),
            vec![stack("diamond", 2), stack("stick", 2)],
            Workbench,
            Tools,
        ),
        recipe(
            "iron_helmet",
            "Iron Helmet",
            stack("iron_helmet", 1),
            vec![stack("iron_ingot", 5)],
            Workbench,
            Combat,
        ),
        recipe(
            "iron_chestplate",
            "Iron Chestplate",
            stack("iron_chestplate", 1),
            vec![stack("iron_ingot", 8)],
            Workbench,
            Combat,
        ),
        recipe(
            "diamond_helmet",
            "Diamond Helmet",
            stack("diamond_helmet", 1),
            vec![stack("diamond", 5)],
            Workbench,
            Combat,
        ),
        recipe(
            "diamond_chestplate",
            "Diamond Chestplate",
            stack("diamond_chestplate", 1),
            vec![stack("diamond", 8)],
            Workbench,
            Combat,
        ),
        recipe(
            "iron_ingot",
            "Smelt Iron",
            stack("iron_ingot", 1),
            vec![stack("raw_iron", 1), stack("coal", 1)],
            Furnace,
            Smelting,
        ),
        recipe(
            "gold_ingot",
            "Smelt Gold",
            stack("gold_ingot", 1),
            vec![stack("raw_gold", 1), stack("coal", 1)],
            Furnace,
            Smelting,
        ),
        recipe(
            "glass",
            "Smelt Glass",
            stack("glass", 1),
            vec![stack("sand", 1), stack("coal", 1)],
            Furnace,
            Smelting,
        ),
        recipe(
            "stone",
            "Smelt Stone",
            stack("stone", 1),
            vec![stack("cobblestone", 1), stack("coal", 1)],
            Furnace,
            Smelting,
        ),
        recipe(
            "charcoal",
            "Charcoal",
            stack("charcoal", 1),
            vec![stack("oak_log", 1)],
            Furnace,
            Smelting,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crafts_planks_from_log() {
        let registry = ItemRegistry::default();
        let recipe_book = RecipeBook::default();
        let recipe = recipe_book
            .recipes
            .iter()
            .find(|recipe| recipe.id == "oak_planks")
            .expect("oak planks recipe should exist");
        let mut inventory = Inventory::new(4, 4);
        inventory.force_add("oak_log", 1);

        try_craft(&mut inventory, recipe, &registry).expect("craft should succeed");

        assert_eq!(inventory.count("oak_log"), 0);
        assert_eq!(inventory.count("oak_planks"), 4);
    }
}
