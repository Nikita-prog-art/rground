use bevy::prelude::*;

use crate::game::ids::ItemId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileKind {
    Grass,
    Dirt,
    Sand,
    Water,
    OakTree,
    BirchTree,
    Stone,
    Cobblestone,
    CoalOre,
    IronOre,
    GoldOre,
    DiamondOre,
    CraftingTable,
    Furnace,
    Chest,
    Torch,
}

#[derive(Clone, Copy, Debug)]
pub struct HarvestDrop {
    pub item: ItemId,
    pub quantity: u32,
    pub replacement: TileKind,
}

impl TileKind {
    pub fn name(self) -> &'static str {
        match self {
            TileKind::Grass => "Grass",
            TileKind::Dirt => "Dirt",
            TileKind::Sand => "Sand",
            TileKind::Water => "Water",
            TileKind::OakTree => "Oak Tree",
            TileKind::BirchTree => "Birch Tree",
            TileKind::Stone => "Stone",
            TileKind::Cobblestone => "Cobblestone",
            TileKind::CoalOre => "Coal Ore",
            TileKind::IronOre => "Iron Ore",
            TileKind::GoldOre => "Gold Ore",
            TileKind::DiamondOre => "Diamond Ore",
            TileKind::CraftingTable => "Crafting Table",
            TileKind::Furnace => "Furnace",
            TileKind::Chest => "Chest",
            TileKind::Torch => "Torch",
        }
    }

    pub fn color(self) -> Color {
        match self {
            TileKind::Grass => Color::srgb(0.20, 0.46, 0.19),
            TileKind::Dirt => Color::srgb(0.39, 0.25, 0.15),
            TileKind::Sand => Color::srgb(0.73, 0.65, 0.38),
            TileKind::Water => Color::srgb(0.09, 0.28, 0.56),
            TileKind::OakTree => Color::srgb(0.08, 0.30, 0.13),
            TileKind::BirchTree => Color::srgb(0.45, 0.63, 0.28),
            TileKind::Stone => Color::srgb(0.34, 0.35, 0.36),
            TileKind::Cobblestone => Color::srgb(0.28, 0.29, 0.30),
            TileKind::CoalOre => Color::srgb(0.18, 0.18, 0.19),
            TileKind::IronOre => Color::srgb(0.53, 0.42, 0.31),
            TileKind::GoldOre => Color::srgb(0.74, 0.61, 0.22),
            TileKind::DiamondOre => Color::srgb(0.18, 0.66, 0.70),
            TileKind::CraftingTable => Color::srgb(0.50, 0.31, 0.13),
            TileKind::Furnace => Color::srgb(0.23, 0.24, 0.25),
            TileKind::Chest => Color::srgb(0.62, 0.39, 0.11),
            TileKind::Torch => Color::srgb(0.98, 0.72, 0.20),
        }
    }

    pub fn is_walkable(self) -> bool {
        matches!(
            self,
            TileKind::Grass | TileKind::Dirt | TileKind::Sand | TileKind::Torch
        )
    }

    pub fn is_station(self) -> bool {
        matches!(self, TileKind::CraftingTable | TileKind::Furnace)
    }

    pub fn harvest_drop(self) -> Option<HarvestDrop> {
        let drop = match self {
            TileKind::Grass => HarvestDrop {
                item: "seeds",
                quantity: 1,
                replacement: TileKind::Dirt,
            },
            TileKind::Dirt => HarvestDrop {
                item: "dirt",
                quantity: 1,
                replacement: TileKind::Grass,
            },
            TileKind::Sand => HarvestDrop {
                item: "sand",
                quantity: 1,
                replacement: TileKind::Grass,
            },
            TileKind::OakTree => HarvestDrop {
                item: "oak_log",
                quantity: 2,
                replacement: TileKind::Grass,
            },
            TileKind::BirchTree => HarvestDrop {
                item: "birch_log",
                quantity: 2,
                replacement: TileKind::Grass,
            },
            TileKind::Stone => HarvestDrop {
                item: "cobblestone",
                quantity: 1,
                replacement: TileKind::Dirt,
            },
            TileKind::Cobblestone => HarvestDrop {
                item: "cobblestone",
                quantity: 1,
                replacement: TileKind::Dirt,
            },
            TileKind::CoalOre => HarvestDrop {
                item: "coal",
                quantity: 2,
                replacement: TileKind::Stone,
            },
            TileKind::IronOre => HarvestDrop {
                item: "raw_iron",
                quantity: 1,
                replacement: TileKind::Stone,
            },
            TileKind::GoldOre => HarvestDrop {
                item: "raw_gold",
                quantity: 1,
                replacement: TileKind::Stone,
            },
            TileKind::DiamondOre => HarvestDrop {
                item: "diamond",
                quantity: 1,
                replacement: TileKind::Stone,
            },
            TileKind::CraftingTable => HarvestDrop {
                item: "crafting_table",
                quantity: 1,
                replacement: TileKind::Grass,
            },
            TileKind::Furnace => HarvestDrop {
                item: "furnace",
                quantity: 1,
                replacement: TileKind::Grass,
            },
            TileKind::Chest => HarvestDrop {
                item: "chest",
                quantity: 1,
                replacement: TileKind::Grass,
            },
            TileKind::Torch => HarvestDrop {
                item: "torch",
                quantity: 1,
                replacement: TileKind::Grass,
            },
            TileKind::Water => return None,
        };

        Some(drop)
    }

    pub fn from_place_item(item: ItemId) -> Option<Self> {
        match item {
            "dirt" => Some(TileKind::Dirt),
            "sand" => Some(TileKind::Sand),
            "cobblestone" => Some(TileKind::Cobblestone),
            "crafting_table" => Some(TileKind::CraftingTable),
            "furnace" => Some(TileKind::Furnace),
            "chest" => Some(TileKind::Chest),
            "torch" => Some(TileKind::Torch),
            _ => None,
        }
    }
}
