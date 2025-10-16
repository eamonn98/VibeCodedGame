use bevy::prelude::*;

use crate::core::TimeScale;
use crate::player::PlayerEntity;
use crate::systems::MovementState;
use crate::systems::{EnemyDeathEvent, EnemyHitEvent};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dummy_enemy).add_systems(
            Update,
            (
                enemy_chase_player,
                apply_enemy_hits,
                tick_enemy_cooldowns,
                update_damage_popups,
            ),
        );
    }
}

#[derive(Component)]
pub struct EnemyEntity;

#[derive(Component, Debug)]
pub struct EnemyHealth {
    pub current: i32,
    pub hurt_cooldown: f32,
}

#[derive(Component)]
struct DamagePopup {
    lifetime: f32,
    initial_lifetime: f32,
    velocity: Vec2,
}

fn spawn_dummy_enemy(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.9, 0.7, 0.2, 1.0),
                custom_size: Some(Vec2::splat(48.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(192.0, 96.0, 1.0),
            ..Default::default()
        },
        MovementState::default(),
        EnemyEntity,
        EnemyHealth {
            current: 3,
            hurt_cooldown: 0.0,
        },
        Name::new("Enemy Dummy"),
    ));
}

fn enemy_chase_player(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    player_query: Query<&Transform, With<PlayerEntity>>,
    mut enemies: Query<(&Transform, &mut MovementState, Option<&EnemyHealth>), With<EnemyEntity>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let dt = (time.delta_seconds() * time_scale.0).max(f32::EPSILON);
    let target = player_transform.translation.truncate();

    for (transform, mut movement, health) in enemies.iter_mut() {
        if let Some(health) = health {
            if health.hurt_cooldown > 0.0 {
                movement.desired_translation = Vec2::ZERO;
                movement.velocity = Vec2::ZERO;
                continue;
            }
        }
        let to_player = target - transform.translation.truncate();
        if to_player.length_squared() < 1.0 {
            movement.desired_translation = Vec2::ZERO;
            movement.velocity = Vec2::ZERO;
            continue;
        }

        let speed = 80.0;
        let desired = to_player.normalize() * speed * dt;
        movement.desired_translation = desired;
        movement.velocity = desired / dt;
        movement.blocked = false;
    }
}

fn apply_enemy_hits(
    mut commands: Commands,
    mut reader: EventReader<EnemyHitEvent>,
    asset_server: Res<AssetServer>,
    mut enemies: Query<(Entity, &Transform, &mut EnemyHealth, Option<&mut Sprite>)>,
    mut death_events: EventWriter<EnemyDeathEvent>,
) {
    for event in reader.read() {
        if let Ok((entity, transform, mut health, sprite)) = enemies.get_mut(event.enemy) {
            if health.current <= 0 {
                continue;
            }

            health.current -= 1;
            health.hurt_cooldown = 0.2;

            if let Some(mut sprite) = sprite {
                sprite.color = Color::srgba(1.0, 0.2, 0.2, 1.0);
            }

            if health.current <= 0 {
                death_events.send(EnemyDeathEvent { enemy: entity });
                commands.entity(entity).despawn_recursive();
            }

            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        "-1",
                        TextStyle {
                            font,
                            font_size: 28.0,
                            color: Color::srgba(1.0, 0.3, 0.3, 1.0),
                        },
                    ),
                    transform: Transform::from_translation(
                        transform.translation + Vec3::new(0.0, 40.0, 2.0),
                    ),
                    ..Default::default()
                },
                DamagePopup {
                    lifetime: 0.5,
                    initial_lifetime: 0.5,
                    velocity: Vec2::new(0.0, 60.0),
                },
                Name::new("Damage Popup"),
            ));
        }
    }
}

fn tick_enemy_cooldowns(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(&mut EnemyHealth, Option<&mut Sprite>)>,
) {
    let dt = time.delta_seconds() * time_scale.0;

    for (mut health, sprite) in &mut query {
        if health.hurt_cooldown > 0.0 {
            health.hurt_cooldown = (health.hurt_cooldown - dt).max(0.0);

            if health.hurt_cooldown <= 0.0 {
                if let Some(mut sprite) = sprite {
                    sprite.color = Color::srgba(0.9, 0.7, 0.2, 1.0);
                }
            }
        }
    }
}

fn update_damage_popups(
    mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Transform, &mut Text, &mut DamagePopup)>,
) {
    let dt = time.delta_seconds() * time_scale.0;

    for (entity, mut transform, mut text, mut popup) in &mut query {
        popup.lifetime -= dt;
        transform.translation += (popup.velocity * dt).extend(0.0);

        let t = (popup.lifetime / popup.initial_lifetime).clamp(0.0, 1.0);
        if let Some(section) = text.sections.first_mut() {
            section.style.color = section.style.color.with_alpha(t);
        }

        if popup.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
