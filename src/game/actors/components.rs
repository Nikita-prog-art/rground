use bevy::prelude::*;

#[derive(Component)]
pub struct Actor;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorKind {
    Player,
    LlmAgent,
    Zombie,
    Skeleton,
    Villager,
}

#[derive(Component, Clone, Debug)]
pub struct ActorName(pub String);

#[derive(Component, Clone, Copy, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MoveIntent {
    pub direction: Vec2,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct WanderSeed(pub u32);
