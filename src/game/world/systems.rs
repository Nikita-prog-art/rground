use bevy::prelude::*;

use crate::game::TILE_SIZE;

use super::map::{TileMap, TileVisual, TileVisuals};

pub fn spawn_world_tiles(
    mut commands: Commands,
    tile_map: Res<TileMap>,
    mut visuals: ResMut<TileVisuals>,
) {
    for pos in tile_map.iter_positions() {
        let Some(tile) = tile_map.get(pos) else {
            continue;
        };

        let mut world_pos = tile_map.tile_to_world(pos);
        world_pos.z = tile_z(tile);

        let entity = commands
            .spawn((
                Sprite::from_color(tile.color(), Vec2::splat(TILE_SIZE - 1.0)),
                Transform::from_translation(world_pos),
                TileVisual,
            ))
            .id();
        visuals.entities.insert(pos, entity);
    }
}

pub fn tile_z(tile: super::tiles::TileKind) -> f32 {
    if tile.is_walkable() { 0.0 } else { 0.05 }
}
