use bevy::prelude::*;

#[derive(Resource)]
pub struct Points(pub i32);

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);