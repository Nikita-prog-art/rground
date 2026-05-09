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

    pub fn is_area_walkable(&self, position: Vec3, half_extent: f32) -> bool {
        let offsets = [
            Vec2::new(-half_extent, -half_extent),
            Vec2::new(-half_extent, half_extent),
            Vec2::new(half_extent, -half_extent),
            Vec2::new(half_extent, half_extent),
        ];

        offsets
            .into_iter()
            .all(|offset| self.is_walkable(self.world_to_tile(position + offset.extend(0.0))))
    }

    pub fn slide_position(&self, position: Vec3, delta: Vec2, half_extent: f32) -> Vec3 {
        let mut result = position;

        if delta.x != 0.0 {
            let candidate = result + Vec3::new(delta.x, 0.0, 0.0);
            if self.is_area_walkable(candidate, half_extent) {
                result.x = candidate.x;
            }
        }

        if delta.y != 0.0 {
            let candidate = result + Vec3::new(0.0, delta.y, 0.0);
            if self.is_area_walkable(candidate, half_extent) {
                result.y = candidate.y;
            }
        }

        result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_walkability_respects_actor_corners() {
        let mut map = TileMap::new(3, 3, TileKind::Grass);
        map.set(IVec2::new(2, 1), TileKind::Stone);

        let center = map.tile_to_world(IVec2::new(1, 1));
        assert!(map.is_area_walkable(center, TILE_SIZE * 0.36));

        let overlapped = center + Vec3::new(TILE_SIZE * 0.15, 0.0, 0.0);
        assert!(!map.is_area_walkable(overlapped, TILE_SIZE * 0.36));
    }
}
