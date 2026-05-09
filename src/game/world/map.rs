use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::{TILE_SIZE, crafting::recipes::CraftStation};

use super::tiles::TileKind;

#[derive(Resource)]
pub struct TileMap {
    pub width: i32,
    pub height: i32,
    tiles: Vec<TileKind>,
}

#[derive(Component)]
pub struct TileVisual;

#[derive(Resource, Default)]
pub struct TileVisuals {
    pub entities: HashMap<IVec2, Entity>,
}

impl TileMap {
    pub fn new(width: i32, height: i32, fill: TileKind) -> Self {
        Self {
            width,
            height,
            tiles: vec![fill; (width * height) as usize],
        }
    }

    pub fn in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width && pos.y < self.height
    }

    pub fn get(&self, pos: IVec2) -> Option<TileKind> {
        self.index(pos).map(|index| self.tiles[index])
    }

    pub fn set(&mut self, pos: IVec2, tile: TileKind) -> bool {
        let Some(index) = self.index(pos) else {
            return false;
        };
        self.tiles[index] = tile;
        true
    }

    pub fn is_walkable(&self, pos: IVec2) -> bool {
        self.get(pos).is_some_and(TileKind::is_walkable)
    }

    pub fn tile_to_world(&self, pos: IVec2) -> Vec3 {
        Vec3::new(
            (pos.x as f32 - self.width as f32 * 0.5) * TILE_SIZE + TILE_SIZE * 0.5,
            (pos.y as f32 - self.height as f32 * 0.5) * TILE_SIZE + TILE_SIZE * 0.5,
            0.0,
        )
    }

    pub fn world_to_tile(&self, position: Vec3) -> IVec2 {
        IVec2::new(
            ((position.x / TILE_SIZE) + self.width as f32 * 0.5).floor() as i32,
            ((position.y / TILE_SIZE) + self.height as f32 * 0.5).floor() as i32,
        )
    }

    pub fn nearest_walkable(&self, origin: IVec2, radius: i32) -> IVec2 {
        for r in 0..=radius {
            for y in -r..=r {
                for x in -r..=r {
                    let candidate = origin + IVec2::new(x, y);
                    if self.is_walkable(candidate) {
                        return candidate;
                    }
                }
            }
        }
        IVec2::new(self.width / 2, self.height / 2)
    }

    pub fn station_available_near(&self, pos: IVec2, station: CraftStation) -> bool {
        if station == CraftStation::Hand {
            return true;
        }

        let expected = match station {
            CraftStation::Hand => return true,
            CraftStation::Workbench => TileKind::CraftingTable,
            CraftStation::Furnace => TileKind::Furnace,
        };

        for y in -2..=2 {
            for x in -2..=2 {
                if expected.is_station() && self.get(pos + IVec2::new(x, y)) == Some(expected) {
                    return true;
                }
            }
        }
        false
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = IVec2> + '_ {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| IVec2::new(x, y)))
    }

    fn index(&self, pos: IVec2) -> Option<usize> {
        if self.in_bounds(pos) {
            Some((pos.y * self.width + pos.x) as usize)
        } else {
            None
        }
    }
}
