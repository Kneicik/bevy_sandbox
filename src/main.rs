mod components;
mod resources;
mod math;

use bevy::{core_pipeline::core_2d::graph::input, ecs::{query, system::assert_is_system}, prelude::*, render::camera::ScalingMode, transform::commands};
use components::*;
use math::*;
use resources::*;
use rand::Rng;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin { 
        min_width: (1280.0), 
        min_height: (720.0) 
    };

    commands.spawn(camera);

    let texture = asset_server.load("wastelands.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        texture,
        ..default()
    });

    let texture = asset_server.load("black_mage.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(86.0, 120.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player {
            speed: 175.0,
            health: 100,
        },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_speed = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::KeyW) && !input.pressed(KeyCode::KeyA) && !input.pressed(KeyCode::KeyD) {
            transform.translation.y += movement_speed;
        }
        if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyA) {
            transform.rotation = Quat::from_rotation_y(0.0);
            transform.translation.y += movement_speed / sqrt(2.0);
            transform.translation.x -= movement_speed / sqrt(2.0);
        }
        if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyD) {
            transform.rotation = Quat::from_rotation_y(180.0 * std::f32::consts::PI / 4.0);
            transform.translation.y += movement_speed / sqrt(2.0);
            transform.translation.x += movement_speed / sqrt(2.0);
        }
        if input.pressed(KeyCode::KeyS) && !input.pressed(KeyCode::KeyA) && !input.pressed(KeyCode::KeyD) {
            transform.translation.y -= movement_speed;
        }
        if input.pressed(KeyCode::KeyS) && input.pressed(KeyCode::KeyA) {
            transform.rotation = Quat::from_rotation_y(0.0);
            transform.translation.y -= movement_speed / sqrt(2.0);
            transform.translation.x -= movement_speed / sqrt(2.0);
        }
        if input.pressed(KeyCode::KeyS) && input.pressed(KeyCode::KeyD) {
            transform.rotation = Quat::from_rotation_y(180.0 * std::f32::consts::PI / 4.0);
            transform.translation.y -= movement_speed / sqrt(2.0);
            transform.translation.x += movement_speed / sqrt(2.0);
        }
        if input.pressed(KeyCode::KeyA) && !input.pressed(KeyCode::KeyW) && !input.pressed(KeyCode::KeyS) {
            transform.rotation = Quat::from_rotation_y(0.0);
            transform.translation.x -= movement_speed;
        }
        if input.pressed(KeyCode::KeyD) && !input.pressed(KeyCode::KeyW) && !input.pressed(KeyCode::KeyS) {
            transform.rotation = Quat::from_rotation_y(180.0 * std::f32::consts::PI / 4.0);
            transform.translation.x += movement_speed;
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(-540.0..540.0);
    let y: f32 = rng.gen_range(-260.0..100.0);

    timer.0.tick(time.delta());

    let timer_finished = timer.0.finished();

    if timer_finished {
        let texture = asset_server.load("cactuar.png");

        commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(x,y,0.0),
                ..default()
            },
            texture,
            ..default()
        },
        Enemy {
            health: 10,
        }));
    }
}

fn despawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut Enemy)>,
    mut points: ResMut<Points>,
) {
    for (enemy_entity, mut enemy) in &mut enemies {
        if enemy.health <= 0 {
            commands.entity(enemy_entity).despawn();
            points.0 += 1;
        }
    }
}

fn main(){
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Copyright infringement™".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(Points(0))
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, spawn_enemy, despawn_enemy))
        .run();
}