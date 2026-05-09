use bevy::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::game::{
    TILE_SIZE, agents::llm::AgentBrain, inventory::model::Inventory, player::components::Facing,
    world::map::TileMap,
};

use super::components::{Actor, ActorKind, ActorName, Health, MoveIntent, WanderSeed};

pub fn spawn_npcs_and_mobs(mut commands: Commands, tile_map: Res<TileMap>) {
    let mut rng = SmallRng::seed_from_u64(0xA61E_1755);

    spawn_group(
        &mut commands,
        &tile_map,
        &mut rng,
        ActorKind::LlmAgent,
        128,
        Color::srgb(0.45, 0.90, 0.98),
        "Agent",
    );
    spawn_group(
        &mut commands,
        &tile_map,
        &mut rng,
        ActorKind::Zombie,
        26,
        Color::srgb(0.18, 0.55, 0.22),
        "Zombie",
    );
    spawn_group(
        &mut commands,
        &tile_map,
        &mut rng,
        ActorKind::Skeleton,
        14,
        Color::srgb(0.78, 0.78, 0.72),
        "Skeleton",
    );
    spawn_group(
        &mut commands,
        &tile_map,
        &mut rng,
        ActorKind::Villager,
        18,
        Color::srgb(0.58, 0.38, 0.78),
        "Villager",
    );
}

fn spawn_group(
    commands: &mut Commands,
    tile_map: &TileMap,
    rng: &mut SmallRng,
    kind: ActorKind,
    count: u32,
    color: Color,
    label: &'static str,
) {
    for i in 0..count {
        let tile = random_walkable_tile(tile_map, rng);
        let mut position = tile_map.tile_to_world(tile);
        position.z = 8.0 + i as f32 * 0.0001;

        let mut entity = commands.spawn((
            Sprite::from_color(color, Vec2::splat(TILE_SIZE * 0.58)),
            Transform::from_translation(position),
            Actor,
            kind,
            ActorName(format!("{label}-{i:03}")),
            Health {
                current: if kind == ActorKind::Villager {
                    20.0
                } else {
                    16.0
                },
                max: if kind == ActorKind::Villager {
                    20.0
                } else {
                    16.0
                },
            },
            MoveIntent::default(),
            Facing(IVec2::Y),
            WanderSeed(rng.random()),
        ));

        if kind == ActorKind::LlmAgent {
            entity.insert((AgentBrain::new(i), Inventory::agent_starter()));
        }
    }
}

fn random_walkable_tile(tile_map: &TileMap, rng: &mut SmallRng) -> IVec2 {
    for _ in 0..512 {
        let x = rng.random_range(2..tile_map.width - 2);
        let y = rng.random_range(2..tile_map.height - 2);
        let pos = IVec2::new(x, y);
        if tile_map.is_walkable(pos) {
            return pos;
        }
    }

    tile_map.nearest_walkable(IVec2::new(tile_map.width / 2, tile_map.height / 2), 16)
}
