use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: i32,
}
#[derive(Component)]
pub struct Enemy {
    pub health:i32,
}

