mod components;
mod resources;
mod math;
use bevy::{core_pipeline::core_2d::graph::input, ecs::{query, system::assert_is_system}, prelude::*, render::camera::ScalingMode, transform::commands, time::Stopwatch};
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
            facing_left: false,
        },
        AttackCooldown {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &mut Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut characters {
        let movement_speed = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::KeyW) && !input.pressed(KeyCode::KeyA) && !input.pressed(KeyCode::KeyD) {
            transform.translation.y += movement_speed;
        }
        if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyA) {
            transform.translation.y += movement_speed / sqrt(2.0);
            transform.translation.x -= movement_speed / sqrt(2.0);
            player.facing_left = true;
        }
        if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyD) {
            transform.translation.y += movement_speed / sqrt(2.0);
            transform.translation.x += movement_speed / sqrt(2.0);
            player.facing_left = false;
        }
        if input.pressed(KeyCode::KeyS) && !input.pressed(KeyCode::KeyA) && !input.pressed(KeyCode::KeyD) {
            transform.translation.y -= movement_speed;
        }
        if input.pressed(KeyCode::KeyS) && input.pressed(KeyCode::KeyA) {
            transform.translation.y -= movement_speed / sqrt(2.0);
            transform.translation.x -= movement_speed / sqrt(2.0);
            player.facing_left = true;
        }
        if input.pressed(KeyCode::KeyS) && input.pressed(KeyCode::KeyD) {
            transform.translation.y -= movement_speed / sqrt(2.0);
            transform.translation.x += movement_speed / sqrt(2.0);
            player.facing_left = false;
        }
        if input.pressed(KeyCode::KeyA) && !input.pressed(KeyCode::KeyW) && !input.pressed(KeyCode::KeyS) {
            transform.translation.x -= movement_speed;
            player.facing_left = true;
        }
        if input.pressed(KeyCode::KeyD) && !input.pressed(KeyCode::KeyW) && !input.pressed(KeyCode::KeyS) {
            transform.translation.x += movement_speed;
            player.facing_left = false;
        }
        if transform.translation.y >= 140.0 {
            transform.translation.y = 140.0
        }
        if player.facing_left == true {
            transform.rotation = Quat::from_rotation_y(0.0);
        } 
        else if player.facing_left == false {
            transform.rotation = Quat::from_rotation_y(180.0 * std::f32::consts::PI / 4.0);
        }
    }
}

fn attack(
    mut commands: Commands,
    mut characters: Query<(&Transform, &mut Player, &mut AttackCooldown)>,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (transform, player, mut cooldown) in &mut characters {

        cooldown.timer.tick(time.delta());

        if input.just_pressed(KeyCode::Space) && cooldown.timer.finished() {
            let texture = asset_server.load("fireball.png");
            let spawn_position = transform.translation;
            let rotation = if player.facing_left {
                Quat::from_rotation_y(std::f32::consts::PI) 
            } else {
                Quat::from_rotation_y(0.0)
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(75.0, 32.5)),
                        ..default()
                    },
                    texture,
                    transform: Transform {
                        translation: spawn_position,
                        rotation,
                        ..default()
                    },
                    ..default()
                },
                Fireball {
                    speed: 400.0,
                    damage: 10,
                    facing_left: player.facing_left,
                },
            ));
            cooldown.timer.reset();
        }
    }
}

fn move_fireball(
    mut query: Query<(&mut Transform, &Fireball)>,
    time: Res<Time>,
) {
    for (mut transform, fireball) in &mut query {
        let fireball_speed = fireball.speed * time.delta_seconds();
        if fireball.facing_left {
            transform.translation.x -= fireball_speed;
        } else {
            transform.translation.x += fireball_speed;
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
            damage: 10,
        }));
    }
}

fn despawn_enemy(
    mut commands: Commands,
    _time: Res<Time>,
    mut enemies: Query<(Entity, &mut Enemy)>,
    mut points: ResMut<Points>,
) {
    for (enemy_entity, enemy) in &mut enemies {
        if enemy.health <= 0 {
            commands.entity(enemy_entity).despawn();
            points.0 += 1;
        }
    }
}

fn check_collisions(
    mut commands: Commands,
    mut fireball_query: Query<(Entity, &Transform, &Fireball)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    mut player_query: Query<(Entity, &Transform, &mut Player)>,
) {
    for (fireball_entity, fireball_transform, fireball) in &mut fireball_query {
        for (enemy_entity, enemy_transform, mut enemy) in &mut enemy_query {
            for (player_entity, player_transform, mut player) in &mut player_query {

                let distance = fireball_transform
                    .translation
                    .distance(enemy_transform.translation);
                let enemy_distance = enemy_transform
                    .translation
                    .distance(player_transform.translation);

                if distance < 40.0 {
                    commands.entity(fireball_entity).despawn();
                    enemy.health -= fireball.damage;
                }
                if enemy_distance < 40.0 {
                    player.health -= enemy.damage;
                }
                if enemy.health <= 0 {
                    commands.entity(enemy_entity).despawn();
                }
                if player.health <= 0 {
                    commands.entity(player_entity).despawn();
                }
            }
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
        .add_systems(Update, (character_movement, spawn_enemy, despawn_enemy, attack))
        .add_systems(StateTransition, (move_fireball, check_collisions))
        .run();
}
