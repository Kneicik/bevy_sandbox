use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: i32,
    pub facing_left: bool,
}

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
    pub damage: i32,
}

#[derive(Component)]
pub struct Fireball {
    pub speed: f32,
    pub damage: i32,
    pub facing_left: bool,
}

#[derive(Component)]
pub struct AttackCooldown {
    pub timer: Timer,
}

