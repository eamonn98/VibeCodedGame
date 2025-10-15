use bevy::ecs::schedule::SystemSet;
use bevy::prelude::*;

use crate::core::{CameraState, InputState, TimeScale};
use crate::player::PlayerEntity;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MovementSystemSet {
    Update,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, MovementSystemSet::Update)
            .init_resource::<MovementSettings>()
            .add_systems(Update, attach_movement_state.in_set(MovementSystemSet::Update))
            .add_systems(Update, update_dash_and_velocity.in_set(MovementSystemSet::Update));
    }
}

#[derive(Resource)]
pub struct MovementSettings {
    pub walk_speed: f32,
    pub dash_speed: f32,
    pub dash_duration: f32,
    pub dash_cooldown: f32,
    pub acceleration: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            walk_speed: 200.0,
            dash_speed: 420.0,
            dash_duration: 0.18,
            dash_cooldown: 0.4,
            acceleration: 12.0,
        }
    }
}

#[derive(Component, Default)]
pub struct MovementState {
    pub velocity: Vec2,
    pub dash_remaining: f32,
    pub cooldown_remaining: f32,
    pub desired_translation: Vec2,
}

fn attach_movement_state(
    mut commands: Commands,
    query: Query<Entity, (With<PlayerEntity>, Without<MovementState>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(MovementState::default());
    }
}

fn update_dash_and_velocity(
    settings: Res<MovementSettings>,
    input_state: Res<InputState>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut camera_state: ResMut<CameraState>,
    mut player_query: Query<&mut MovementState, With<PlayerEntity>>,
) {
    let dt = time.delta_seconds() * time_scale.0;

    let mut dash_triggered = false;
    for mut state in &mut player_query {
        if state.dash_remaining > 0.0 {
            state.dash_remaining = (state.dash_remaining - dt).max(0.0);
        }

        if state.cooldown_remaining > 0.0 {
            state.cooldown_remaining = (state.cooldown_remaining - dt).max(0.0);
        }

        if input_state.dash_just_pressed
            && input_state.movement.length_squared() > 0.0
            && state.cooldown_remaining <= 0.0
        {
            state.dash_remaining = settings.dash_duration;
            state.cooldown_remaining = settings.dash_cooldown;
            dash_triggered = true;
        }

        let target_speed = if state.dash_remaining > 0.0 {
            settings.dash_speed
        } else {
            settings.walk_speed
        };

        let target_velocity = input_state.movement * target_speed;
        let lerp_factor = (settings.acceleration * dt).clamp(0.0, 1.0);
        state.velocity = state.velocity.lerp(target_velocity, lerp_factor);
        state.desired_translation = state.velocity * dt;
    }

    if dash_triggered {
        camera_state.desired_shake = Vec2::splat(6.0);
    }
}
