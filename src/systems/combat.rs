use bevy::prelude::*;

use crate::core::{CameraState, InputState, TimeScale};
use crate::enemy::EnemyEntity;
use crate::player::PlayerEntity;
use crate::systems::MovementState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_event::<EnemyHitEvent>()
            .init_resource::<AttackSettings>()
            .init_resource::<AttackState>()
            .add_systems(
                Update,
                (
                    queue_attack_inputs,
                    resolve_attack_state,
                    spawn_attack_hitboxes,
                    detect_attack_collisions,
                    update_enemy_hit_flash,
                    apply_attack_feedback,
                    expire_attack_hitboxes,
                ),
            );
    }
}

#[derive(Resource)]
pub struct AttackSettings {
    pub windup_seconds: f32,
    pub active_seconds: f32,
    pub recovery_seconds: f32,
}

impl Default for AttackSettings {
    fn default() -> Self {
        Self {
            windup_seconds: 0.12,
            active_seconds: 0.1,
            recovery_seconds: 0.25,
        }
    }
}

#[derive(Resource, Default)]
pub struct AttackState {
    pub timer: f32,
    pub phase: AttackPhase,
    pub queued: bool,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AttackPhase {
    #[default]
    Idle,
    Windup,
    Active,
    Recovery,
}

#[derive(Event, Clone, Copy)]
pub struct AttackEvent {
    pub phase: AttackPhase,
}

#[derive(Component)]
struct AttackHitbox {
    lifetime: f32,
    radius: f32,
    hit_entities: Vec<Entity>,
}

#[derive(Component)]
struct EnemyHitFlash {
    timer: f32,
}

fn queue_attack_inputs(
    input: Res<InputState>,
    mut state: ResMut<AttackState>,
    query: Query<Entity, With<PlayerEntity>>,
) {
    if query.get_single().is_err() {
        return;
    }

    if input.attack {
        match state.phase {
            AttackPhase::Idle | AttackPhase::Recovery => {
                if !state.queued {
                    state.queued = true;
                }
            }
            _ => {}
        }
    }
}

fn resolve_attack_state(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    settings: Res<AttackSettings>,
    mut state: ResMut<AttackState>,
    player_query: Query<Entity, With<PlayerEntity>>,
    mut events: EventWriter<AttackEvent>,
) {
    if player_query.get_single().is_err() {
        state.phase = AttackPhase::Idle;
        state.timer = 0.0;
        state.queued = false;
        return;
    }

    let dt = time.delta_seconds() * time_scale.0;

    match state.phase {
        AttackPhase::Idle => {
            if state.queued {
                state.queued = false;
                state.phase = AttackPhase::Windup;
                state.timer = settings.windup_seconds;
                events.send(AttackEvent {
                    phase: AttackPhase::Windup,
                });
            }
        }
        AttackPhase::Windup => {
            state.timer -= dt;
            if state.timer <= 0.0 {
                state.phase = AttackPhase::Active;
                state.timer = settings.active_seconds;
                events.send(AttackEvent {
                    phase: AttackPhase::Active,
                });
            }
        }
        AttackPhase::Active => {
            state.timer -= dt;
            if state.timer <= 0.0 {
                state.phase = AttackPhase::Recovery;
                state.timer = settings.recovery_seconds;
                events.send(AttackEvent {
                    phase: AttackPhase::Recovery,
                });
            }
        }
        AttackPhase::Recovery => {
            state.timer -= dt;
            if state.timer <= 0.0 {
                state.phase = AttackPhase::Idle;
                events.send(AttackEvent {
                    phase: AttackPhase::Idle,
                });
            }
        }
    }
}

fn spawn_attack_hitboxes(
    mut commands: Commands,
    mut reader: EventReader<AttackEvent>,
    player_query: Query<(&Transform, Option<&MovementState>), With<PlayerEntity>>,
) {
    let Ok((player_transform, movement_state)) = player_query.get_single() else {
        reader.clear();
        return;
    };

    for event in reader.read() {
        if event.phase != AttackPhase::Active {
            continue;
        }

        let facing = movement_state
            .and_then(|state| {
                if state.velocity.length_squared() > 0.01 {
                    Some(state.velocity.normalize())
                } else {
                    None
                }
            })
            .unwrap_or(Vec2::X);

        let hitbox_offset = facing * 40.0;
        let translation = player_transform.translation + hitbox_offset.extend(0.1);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.4, 0.2, 0.8),
                    custom_size: Some(Vec2::splat(32.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
            AttackHitbox {
                lifetime: 0.12,
                radius: 20.0,
                hit_entities: Vec::new(),
            },
            Name::new("Attack Hitbox"),
        ));
    }
}

#[derive(Event, Clone, Copy)]
pub struct EnemyHitEvent {
    pub enemy: Entity,
}

fn detect_attack_collisions(
    mut commands: Commands,
    mut hitboxes: Query<(&Transform, &mut AttackHitbox)>,
    enemies: Query<(Entity, &Transform), With<EnemyEntity>>,
    mut events: EventWriter<EnemyHitEvent>,
) {
    for (hitbox_transform, mut hitbox) in &mut hitboxes {
        let hitbox_center = hitbox_transform.translation.truncate();

        for (enemy_entity, enemy_transform) in &enemies {
            if hitbox.hit_entities.contains(&enemy_entity) {
                continue;
            }

            let enemy_center = enemy_transform.translation.truncate();
            let distance = hitbox_center.distance(enemy_center);

            const ENEMY_RADIUS: f32 = 24.0;
            if distance <= hitbox.radius + ENEMY_RADIUS {
                hitbox.hit_entities.push(enemy_entity);
                commands
                    .entity(enemy_entity)
                    .insert(EnemyHitFlash { timer: 0.18 });
                events.send(EnemyHitEvent {
                    enemy: enemy_entity,
                });
            }
        }
    }
}

fn update_enemy_hit_flash(
    mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Sprite, &mut EnemyHitFlash)>,
) {
    let dt = time.delta_seconds() * time_scale.0;

    for (entity, mut sprite, mut flash) in &mut query {
        flash.timer -= dt;
        if flash.timer <= 0.0 {
            sprite.color = Color::srgba(0.9, 0.7, 0.2, 1.0);
            commands.entity(entity).remove::<EnemyHitFlash>();
        } else {
            let intensity = (flash.timer / 0.18).clamp(0.0, 1.0);
            sprite.color = Color::srgba(1.0, 0.3 + 0.4 * intensity, 0.3, 1.0);
        }
    }
}

fn apply_attack_feedback(
    mut reader: EventReader<EnemyHitEvent>,
    mut camera_state: ResMut<CameraState>,
) {
    let mut any = false;
    for _ in reader.read() {
        any = true;
    }

    if any {
        camera_state.desired_shake = camera_state.desired_shake.max(Vec2::splat(4.0));
    }
}

fn expire_attack_hitboxes(
    mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut AttackHitbox)>,
) {
    let dt = time.delta_seconds() * time_scale.0;

    for (entity, mut hitbox) in &mut query {
        hitbox.lifetime -= dt;
        if hitbox.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
