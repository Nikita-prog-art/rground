use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::{ids::ItemId, world::tiles::TileKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
    Block,
    Station,
    Material,
    Tool,
    Weapon,
    Food,
    Armor,
    Utility,
}

#[derive(Clone, Debug)]
pub struct ItemDef {
    pub id: ItemId,
    pub name: &'static str,
    pub kind: ItemKind,
    pub max_stack: u32,
    pub place_tile: Option<TileKind>,
}

#[derive(Resource)]
pub struct ItemRegistry {
    defs: HashMap<ItemId, ItemDef>,
    ordered: Vec<ItemId>,
}

impl Default for ItemRegistry {
    fn default() -> Self {
        let mut registry = Self {
            defs: HashMap::new(),
            ordered: Vec::new(),
        };

        for def in minecraft_like_items() {
            registry.register(def);
        }

        registry
    }
}

impl ItemRegistry {
    pub fn register(&mut self, def: ItemDef) {
        self.ordered.push(def.id);
        self.defs.insert(def.id, def);
    }

    pub fn get(&self, id: ItemId) -> Option<&ItemDef> {
        self.defs.get(id)
    }

    pub fn name(&self, id: ItemId) -> &'static str {
        self.get(id).map_or(id, |def| def.name)
    }

    pub fn max_stack(&self, id: ItemId) -> u32 {
        self.get(id).map_or(64, |def| def.max_stack)
    }

    pub fn all_ids(&self) -> &[ItemId] {
        &self.ordered
    }

    pub fn resolve_id(&self, value: &str) -> Option<ItemId> {
        self.defs.get_key_value(value).map(|(id, _)| *id)
    }
}

fn item(
    id: ItemId,
    name: &'static str,
    kind: ItemKind,
    max_stack: u32,
    place_tile: Option<TileKind>,
) -> ItemDef {
    ItemDef {
        id,
        name,
        kind,
        max_stack,
        place_tile,
    }
}

fn minecraft_like_items() -> Vec<ItemDef> {
    use ItemKind::*;

    vec![
        item("grass_block", "Grass Block", Block, 64, None),
        item("dirt", "Dirt", Block, 64, Some(TileKind::Dirt)),
        item("coarse_dirt", "Coarse Dirt", Block, 64, None),
        item("sand", "Sand", Block, 64, Some(TileKind::Sand)),
        item("gravel", "Gravel", Block, 64, None),
        item(
            "cobblestone",
            "Cobblestone",
            Block,
            64,
            Some(TileKind::Cobblestone),
        ),
        item("stone", "Stone", Block, 64, Some(TileKind::Stone)),
        item("deepslate", "Deepslate", Block, 64, None),
        item("oak_log", "Oak Log", Block, 64, None),
        item("birch_log", "Birch Log", Block, 64, None),
        item("spruce_log", "Spruce Log", Block, 64, None),
        item("oak_planks", "Oak Planks", Block, 64, None),
        item("birch_planks", "Birch Planks", Block, 64, None),
        item("spruce_planks", "Spruce Planks", Block, 64, None),
        item(
            "crafting_table",
            "Crafting Table",
            Station,
            64,
            Some(TileKind::CraftingTable),
        ),
        item("furnace", "Furnace", Station, 64, Some(TileKind::Furnace)),
        item("chest", "Chest", Station, 64, Some(TileKind::Chest)),
        item("torch", "Torch", Utility, 64, Some(TileKind::Torch)),
        item("glass", "Glass", Block, 64, None),
        item("white_wool", "White Wool", Block, 64, None),
        item("bed", "Bed", Utility, 1, None),
        item("oak_door", "Oak Door", Utility, 64, None),
        item("oak_fence", "Oak Fence", Block, 64, None),
        item("ladder", "Ladder", Utility, 64, None),
        item("coal_ore", "Coal Ore", Block, 64, None),
        item("iron_ore", "Iron Ore", Block, 64, None),
        item("gold_ore", "Gold Ore", Block, 64, None),
        item("diamond_ore", "Diamond Ore", Block, 64, None),
        item("coal", "Coal", Material, 64, None),
        item("charcoal", "Charcoal", Material, 64, None),
        item("raw_iron", "Raw Iron", Material, 64, None),
        item("raw_gold", "Raw Gold", Material, 64, None),
        item("iron_ingot", "Iron Ingot", Material, 64, None),
        item("gold_ingot", "Gold Ingot", Material, 64, None),
        item("copper_ingot", "Copper Ingot", Material, 64, None),
        item("diamond", "Diamond", Material, 64, None),
        item("emerald", "Emerald", Material, 64, None),
        item("redstone", "Redstone Dust", Material, 64, None),
        item("lapis_lazuli", "Lapis Lazuli", Material, 64, None),
        item("stick", "Stick", Material, 64, None),
        item("flint", "Flint", Material, 64, None),
        item("string", "String", Material, 64, None),
        item("feather", "Feather", Material, 64, None),
        item("bone", "Bone", Material, 64, None),
        item("leather", "Leather", Material, 64, None),
        item("paper", "Paper", Material, 64, None),
        item("book", "Book", Material, 64, None),
        item("wheat", "Wheat", Material, 64, None),
        item("seeds", "Seeds", Material, 64, None),
        item("apple", "Apple", Food, 64, None),
        item("bread", "Bread", Food, 64, None),
        item("carrot", "Carrot", Food, 64, None),
        item("potato", "Potato", Food, 64, None),
        item("arrow", "Arrow", Utility, 64, None),
        item("bucket", "Bucket", Utility, 16, None),
        item("shield", "Shield", Utility, 1, None),
        item("bow", "Bow", Weapon, 1, None),
        item("wooden_sword", "Wooden Sword", Weapon, 1, None),
        item("stone_sword", "Stone Sword", Weapon, 1, None),
        item("iron_sword", "Iron Sword", Weapon, 1, None),
        item("diamond_sword", "Diamond Sword", Weapon, 1, None),
        item("wooden_pickaxe", "Wooden Pickaxe", Tool, 1, None),
        item("stone_pickaxe", "Stone Pickaxe", Tool, 1, None),
        item("iron_pickaxe", "Iron Pickaxe", Tool, 1, None),
        item("diamond_pickaxe", "Diamond Pickaxe", Tool, 1, None),
        item("wooden_axe", "Wooden Axe", Tool, 1, None),
        item("stone_axe", "Stone Axe", Tool, 1, None),
        item("iron_axe", "Iron Axe", Tool, 1, None),
        item("diamond_axe", "Diamond Axe", Tool, 1, None),
        item("wooden_shovel", "Wooden Shovel", Tool, 1, None),
        item("stone_shovel", "Stone Shovel", Tool, 1, None),
        item("iron_shovel", "Iron Shovel", Tool, 1, None),
        item("diamond_shovel", "Diamond Shovel", Tool, 1, None),
        item("wooden_hoe", "Wooden Hoe", Tool, 1, None),
        item("stone_hoe", "Stone Hoe", Tool, 1, None),
        item("iron_hoe", "Iron Hoe", Tool, 1, None),
        item("diamond_hoe", "Diamond Hoe", Tool, 1, None),
        item("leather_helmet", "Leather Helmet", Armor, 1, None),
        item("iron_helmet", "Iron Helmet", Armor, 1, None),
        item("diamond_helmet", "Diamond Helmet", Armor, 1, None),
        item("iron_chestplate", "Iron Chestplate", Armor, 1, None),
        item("diamond_chestplate", "Diamond Chestplate", Armor, 1, None),
    ]
}
