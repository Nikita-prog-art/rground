use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Copy, Debug)]
pub struct Facing(pub IVec2);
