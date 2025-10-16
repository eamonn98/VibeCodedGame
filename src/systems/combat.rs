use bevy::audio::{AudioBundle, AudioSource, PlaybackSettings};
use bevy::prelude::*;

use crate::core::{CameraState, InputState, TimeScale};
use crate::enemy::EnemyEntity;
use crate::player::PlayerEntity;
use crate::systems::MovementState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        let hit_sfx = {
            let asset_server = app.world().resource::<AssetServer>();
            [
                "audio/switch17.ogg",
                "audio/switch23.ogg",
                "audio/switch36.ogg",
                "audio/click4.ogg",
            ]
            .iter()
            .map(|path| asset_server.load(*path))
            .collect::<Vec<_>>()
        };

        app.insert_resource(CombatAudio {
            hits: hit_sfx,
            next_index: 0,
        })
        .add_event::<AttackEvent>()
        .add_event::<EnemyHitEvent>()
        .add_event::<EnemyDeathEvent>()
        .init_resource::<AttackSettings>()
        .init_resource::<AttackState>()
        .init_resource::<ComboTracker>()
        .add_systems(
            Update,
            (
                queue_attack_inputs,
                resolve_attack_state,
                spawn_attack_hitboxes,
                detect_attack_collisions,
                update_enemy_hit_flash,
                spawn_hit_sparks,
                update_hit_sparks,
                apply_attack_feedback,
                accumulate_combo_hits,
                reset_combo_on_death,
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

#[derive(Component)]
struct HitSpark {
    lifetime: f32,
    initial_lifetime: f32,
}

#[derive(Event, Clone, Copy)]
pub struct EnemyHitEvent {
    pub enemy: Entity,
}

#[derive(Event, Clone)]
pub struct EnemyDeathEvent {
    pub enemy: Entity,
    pub name: Option<String>,
}

#[derive(Resource, Default)]
pub struct ComboTracker {
    current: u32,
    best: u32,
    decay_timer: f32,
    last_defeated_entity: Option<Entity>,
    last_defeated_name: Option<String>,
    last_defeated_time: Option<f32>,
}

impl ComboTracker {
    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn best(&self) -> u32 {
        self.best
    }

    pub fn is_active(&self) -> bool {
        self.current > 0
    }

    pub fn last_defeated_entity(&self) -> Option<Entity> {
        self.last_defeated_entity
    }

    pub fn last_defeated_name(&self) -> Option<&str> {
        self.last_defeated_name.as_deref()
    }

    pub fn last_defeated_elapsed(&self, now: f32) -> Option<f32> {
        self.last_defeated_time.map(|t| (now - t).max(0.0))
    }
}

#[derive(Resource)]
struct CombatAudio {
    hits: Vec<Handle<AudioSource>>,
    next_index: usize,
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

fn detect_attack_collisions(
    mut commands: Commands,
    mut hitboxes: Query<(&Transform, &mut AttackHitbox)>,
    enemies: Query<(Entity, &Transform), With<EnemyEntity>>,
    mut hit_events: EventWriter<EnemyHitEvent>,
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
                hit_events.send(EnemyHitEvent {
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

fn spawn_hit_sparks(
    mut commands: Commands,
    mut reader: EventReader<EnemyHitEvent>,
    query: Query<&Transform, With<EnemyEntity>>,
) {
    for event in reader.read() {
        if let Ok(transform) = query.get(event.enemy) {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(1.0, 0.8, 0.3, 0.9),
                        custom_size: Some(Vec2::splat(20.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        transform.translation + Vec3::new(0.0, 24.0, 1.5),
                    ),
                    ..Default::default()
                },
                HitSpark {
                    lifetime: 0.18,
                    initial_lifetime: 0.18,
                },
                Name::new("Hit Spark"),
            ));
        }
    }
}

fn update_hit_sparks(
    mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut HitSpark)>,
) {
    let dt = time.delta_seconds() * time_scale.0;

    for (entity, mut transform, mut sprite, mut spark) in &mut query {
        spark.lifetime -= dt;
        transform.scale *= Vec3::splat(1.0 + dt * 4.0);

        let t = (spark.lifetime / spark.initial_lifetime).clamp(0.0, 1.0);
        sprite.color = sprite.color.with_alpha(t);

        if spark.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn apply_attack_feedback(
    mut commands: Commands,
    mut reader: EventReader<EnemyHitEvent>,
    mut camera_state: ResMut<CameraState>,
    mut audio: ResMut<CombatAudio>,
) {
    let mut any = false;
    for _ in reader.read() {
        any = true;
    }

    if any && !audio.hits.is_empty() {
        camera_state.desired_shake = camera_state.desired_shake.max(Vec2::splat(4.0));
        let index = audio.next_index % audio.hits.len();
        let handle = audio.hits[index].clone();
        audio.next_index = (index + 1) % audio.hits.len();
        commands.spawn((
            AudioBundle {
                source: handle,
                settings: PlaybackSettings::DESPAWN,
            },
            Name::new("Hit SFX"),
        ));
    }
}

fn accumulate_combo_hits(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut tracker: ResMut<ComboTracker>,
    mut reader: EventReader<EnemyHitEvent>,
) {
    let dt = time.delta_seconds() * time_scale.0;
    tracker.decay_timer = (tracker.decay_timer - dt).max(0.0);

    let mut hit_registered = false;
    for _ in reader.read() {
        hit_registered = true;
    }

    if hit_registered {
        if tracker.decay_timer <= 0.0 {
            tracker.current = 0;
            tracker.last_defeated_entity = None;
            tracker.last_defeated_name = None;
            tracker.last_defeated_time = None;
        }

        tracker.current = tracker.current.saturating_add(1);
        tracker.decay_timer = 1.5;
        tracker.best = tracker.best.max(tracker.current);
    } else if tracker.decay_timer <= 0.0 && tracker.current > 0 {
        tracker.current = 0;
        tracker.last_defeated_entity = None;
        tracker.last_defeated_name = None;
        tracker.last_defeated_time = None;
    }
}

fn reset_combo_on_death(
    time: Res<Time>,
    mut tracker: ResMut<ComboTracker>,
    mut death_reader: EventReader<EnemyDeathEvent>,
) {
    let mut saw_death = false;
    let now = time.elapsed_seconds();

    for event in death_reader.read() {
        tracker.last_defeated_entity = Some(event.enemy);
        tracker.last_defeated_name = event.name.clone();
        tracker.last_defeated_time = Some(now);
        saw_death = true;
    }

    if saw_death && tracker.current > 0 {
        tracker.current = 0;
        tracker.decay_timer = 0.0;
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
