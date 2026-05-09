use bevy::prelude::*;

use crate::game::{ACTOR_COLLISION_HALF_EXTENT, player::components::Player, world::map::TileMap};

use super::components::{ActorKind, MoveIntent, WanderSeed};

#[derive(Resource)]
pub struct MobAiClock {
    timer: Timer,
}

impl Default for MobAiClock {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.22, TimerMode::Repeating),
        }
    }
}

pub fn mob_ai_system(
    time: Res<Time>,
    mut clock: ResMut<MobAiClock>,
    player: Query<&Transform, With<Player>>,
    mut actors: Query<(&Transform, &ActorKind, &WanderSeed, &mut MoveIntent), Without<Player>>,
) {
    clock.timer.tick(time.delta());
    if !clock.timer.just_finished() {
        return;
    }

    let player_pos = player.single().map(|transform| transform.translation).ok();

    for (transform, kind, seed, mut intent) in &mut actors {
        match kind {
            ActorKind::Zombie => {
                intent.direction = chase_player(transform.translation, player_pos, 320.0)
                    .unwrap_or_else(|| wander_direction(seed.0, time.elapsed_secs_wrapped()));
            }
            ActorKind::Skeleton => {
                intent.direction = kite_player(transform.translation, player_pos, 180.0, 420.0)
                    .unwrap_or_else(|| wander_direction(seed.0, time.elapsed_secs_wrapped()));
            }
            ActorKind::Villager => {
                intent.direction = wander_direction(seed.0, time.elapsed_secs_wrapped() * 0.45);
            }
            ActorKind::LlmAgent | ActorKind::Player => {}
        }
    }
}

pub fn mob_motion_system(
    time: Res<Time>,
    tile_map: Res<TileMap>,
    mut actors: Query<(&mut Transform, &ActorKind, &MoveIntent), Without<Player>>,
) {
    for (mut transform, kind, intent) in &mut actors {
        if *kind == ActorKind::LlmAgent || intent.direction == Vec2::ZERO {
            continue;
        }

        let speed = match kind {
            ActorKind::Zombie => 86.0,
            ActorKind::Skeleton => 78.0,
            ActorKind::Villager => 48.0,
            _ => 64.0,
        };

        let direction = intent.direction.normalize_or_zero();
        let delta = direction * speed * time.delta_secs();
        transform.translation =
            tile_map.slide_position(transform.translation, delta, ACTOR_COLLISION_HALF_EXTENT);
    }
}

fn chase_player(position: Vec3, player_pos: Option<Vec3>, radius: f32) -> Option<Vec2> {
    let player_pos = player_pos?;
    let delta = (player_pos - position).truncate();
    if delta.length_squared() < radius * radius {
        Some(delta.normalize_or_zero())
    } else {
        None
    }
}

fn kite_player(position: Vec3, player_pos: Option<Vec3>, ideal: f32, radius: f32) -> Option<Vec2> {
    let player_pos = player_pos?;
    let delta = (player_pos - position).truncate();
    let distance = delta.length();
    if distance > radius {
        None
    } else if distance < ideal {
        Some(-delta.normalize_or_zero())
    } else {
        Some(delta.normalize_or_zero())
    }
}

fn wander_direction(seed: u32, t: f32) -> Vec2 {
    let phase = seed as f32 * 0.000_001 + t;
    let x = (phase * 1.7).sin();
    let y = (phase * 1.13).cos();
    let direction = Vec2::new(x, y);
    if direction.length_squared() < 0.01 {
        Vec2::ZERO
    } else {
        direction.normalize_or_zero()
    }
}
