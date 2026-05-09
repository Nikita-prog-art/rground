use bevy::prelude::*;

use super::{map::TileMap, tiles::TileKind};

pub fn generate_world(width: i32, height: i32) -> TileMap {
    let mut map = TileMap::new(width, height, TileKind::Grass);

    for y in 0..height {
        for x in 0..width {
            let pos = IVec2::new(x, y);
            let water = noise(x, y, 11);
            let forest = noise(x, y, 71);
            let ridge = noise(x / 2, y / 2, 131);
            let ore = noise(x, y, 311);
            let edge_bias = distance_to_center(x, y, width, height);

            let tile = if water + edge_bias * 0.12 < 0.16 {
                TileKind::Water
            } else if water < 0.22 {
                TileKind::Sand
            } else if ridge > 0.83 {
                if ore > 0.985 {
                    TileKind::DiamondOre
                } else if ore > 0.965 {
                    TileKind::GoldOre
                } else if ore > 0.925 {
                    TileKind::IronOre
                } else if ore > 0.870 {
                    TileKind::CoalOre
                } else {
                    TileKind::Stone
                }
            } else if forest > 0.875 {
                TileKind::OakTree
            } else if forest > 0.835 {
                TileKind::BirchTree
            } else if forest < 0.045 {
                TileKind::Dirt
            } else {
                TileKind::Grass
            };

            map.set(pos, tile);
        }
    }

    let center = IVec2::new(width / 2, height / 2);
    for y in -3..=3 {
        for x in -3..=3 {
            map.set(center + IVec2::new(x, y), TileKind::Grass);
        }
    }
    map.set(center + IVec2::new(2, 0), TileKind::CraftingTable);
    map.set(center + IVec2::new(3, 0), TileKind::Furnace);
    map.set(center + IVec2::new(0, 3), TileKind::Chest);

    map
}

fn noise(x: i32, y: i32, seed: u32) -> f32 {
    let mut n = x as u32;
    n = n.wrapping_mul(0x85eb_ca6b);
    n ^= y as u32 + seed.wrapping_mul(0xc2b2_ae35);
    n ^= n >> 16;
    n = n.wrapping_mul(0x7feb_352d);
    n ^= n >> 15;
    n = n.wrapping_mul(0x846c_a68b);
    n ^= n >> 16;
    (n as f32) / (u32::MAX as f32)
}

fn distance_to_center(x: i32, y: i32, width: i32, height: i32) -> f32 {
    let dx = x as f32 / width as f32 - 0.5;
    let dy = y as f32 / height as f32 - 0.5;
    (dx * dx + dy * dy).sqrt().min(1.0)
}
