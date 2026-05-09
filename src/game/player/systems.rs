use bevy::prelude::*;

use crate::game::{
    ACTOR_COLLISION_HALF_EXTENT, TILE_SIZE,
    actors::components::{Actor, ActorKind, ActorName, Health},
    inventory::model::Inventory,
    items::registry::ItemRegistry,
    world::{
        map::{TileMap, TileVisual, TileVisuals},
        systems::tile_z,
        tiles::TileKind,
    },
};

use super::components::{Facing, Player};

const PLAYER_SPEED: f32 = 220.0;

pub fn spawn_player(mut commands: Commands, tile_map: Res<TileMap>) {
    let spawn_tile =
        tile_map.nearest_walkable(IVec2::new(tile_map.width / 2, tile_map.height / 2), 12);
    let mut position = tile_map.tile_to_world(spawn_tile);
    position.z = 10.0;

    commands.spawn((
        Sprite::from_color(Color::srgb(0.16, 0.70, 0.92), Vec2::splat(TILE_SIZE * 0.72)),
        Transform::from_translation(position),
        Player,
        Actor,
        ActorName("Player".to_string()),
        ActorKind::Player,
        Health {
            current: 20.0,
            max: 20.0,
        },
        Facing(IVec2::Y),
        Inventory::starter(),
    ));
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    tile_map: Res<TileMap>,
    mut players: Query<(&mut Transform, &mut Facing), With<Player>>,
) {
    let Ok((mut transform, mut facing)) = players.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction == Vec2::ZERO {
        return;
    }

    let normalized = direction.normalize();
    facing.0 = dominant_axis(normalized);

    let delta = normalized * PLAYER_SPEED * time.delta_secs();
    transform.translation =
        tile_map.slide_position(transform.translation, delta, ACTOR_COLLISION_HALF_EXTENT);
}

pub fn harvest_front_tile_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut tile_map: ResMut<TileMap>,
    visuals: Res<TileVisuals>,
    registry: Res<ItemRegistry>,
    mut tile_sprites: Query<(&mut Sprite, &mut Transform), With<TileVisual>>,
    mut players: Query<(&Transform, &Facing, &mut Inventory), (With<Player>, Without<TileVisual>)>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok((transform, facing, mut inventory)) = players.single_mut() else {
        return;
    };

    let target = tile_map.world_to_tile(transform.translation) + facing.0;
    harvest_tile(
        target,
        &mut tile_map,
        &visuals,
        &registry,
        &mut tile_sprites,
        &mut inventory,
    );
}

pub fn place_selected_tile_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut tile_map: ResMut<TileMap>,
    visuals: Res<TileVisuals>,
    registry: Res<ItemRegistry>,
    mut tile_sprites: Query<(&mut Sprite, &mut Transform), With<TileVisual>>,
    mut players: Query<(&Transform, &Facing, &mut Inventory), (With<Player>, Without<TileVisual>)>,
) {
    if !keyboard.just_pressed(KeyCode::KeyF) {
        return;
    }

    let Ok((transform, facing, mut inventory)) = players.single_mut() else {
        return;
    };

    let Some(stack) = inventory.selected_stack().cloned() else {
        return;
    };
    let Some(def) = registry.get(stack.item) else {
        return;
    };
    let Some(tile) = def
        .place_tile
        .or_else(|| TileKind::from_place_item(stack.item))
    else {
        return;
    };

    let target = tile_map.world_to_tile(transform.translation) + facing.0;
    if !tile_map.in_bounds(target) || !tile_map.get(target).is_some_and(TileKind::is_walkable) {
        return;
    }

    if !inventory.remove(stack.item, 1) {
        return;
    }

    set_tile_visual(target, tile, &mut tile_map, &visuals, &mut tile_sprites);
}

pub fn harvest_tile(
    target: IVec2,
    tile_map: &mut TileMap,
    visuals: &TileVisuals,
    registry: &ItemRegistry,
    tile_sprites: &mut Query<(&mut Sprite, &mut Transform), With<TileVisual>>,
    inventory: &mut Inventory,
) -> bool {
    let Some(tile) = tile_map.get(target) else {
        return false;
    };
    let Some(drop) = tile.harvest_drop() else {
        return false;
    };

    if !inventory.can_add(drop.item, drop.quantity, registry) {
        return false;
    }

    inventory.add(drop.item, drop.quantity, registry);
    set_tile_visual(target, drop.replacement, tile_map, visuals, tile_sprites);
    true
}

pub fn set_tile_visual(
    target: IVec2,
    tile: TileKind,
    tile_map: &mut TileMap,
    visuals: &TileVisuals,
    tile_sprites: &mut Query<(&mut Sprite, &mut Transform), With<TileVisual>>,
) {
    tile_map.set(target, tile);
    let Some(entity) = visuals.entities.get(&target) else {
        return;
    };
    let Ok((mut sprite, mut transform)) = tile_sprites.get_mut(*entity) else {
        return;
    };
    sprite.color = tile.color();
    transform.translation.z = tile_z(tile);
}

fn dominant_axis(direction: Vec2) -> IVec2 {
    if direction.x.abs() > direction.y.abs() {
        if direction.x > 0.0 {
            IVec2::X
        } else {
            IVec2::NEG_X
        }
    } else if direction.y > 0.0 {
        IVec2::Y
    } else {
        IVec2::NEG_Y
    }
}
