use bevy::prelude::*;

use crate::config::AppConfig;
use crate::core::InputState;
use crate::player::PlayerEntity;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .init_resource::<CameraState>()
            .add_systems(Startup, setup_camera)
            .add_systems(PostStartup, align_initial_camera)
            .add_systems(Update, (toggle_follow, follow_player, apply_shake));
    }
}

#[derive(Resource)]
pub struct CameraSettings {
    pub follow_damping: f32,
    pub max_distance: f32,
    pub shake_decay: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            follow_damping: 8.0,
            max_distance: 200.0,
            shake_decay: 3.0,
        }
    }
}

#[derive(Resource)]
pub struct CameraState {
    pub follow_enabled: bool,
    pub shake_offset: Vec2,
    pub desired_shake: Vec2,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            follow_enabled: true,
            shake_offset: Vec2::ZERO,
            desired_shake: Vec2::ZERO,
        }
    }
}

fn setup_camera(mut commands: Commands, config: Res<AppConfig>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..Default::default()
            },
            projection: OrthographicProjection {
                scale: 1.0,
                ..Default::default()
            },
            ..Default::default()
        },
        Name::new(format!("{} Camera", config.window_title)),
    ));
}

fn toggle_follow(input_state: Res<InputState>, mut camera_state: ResMut<CameraState>) {
    if input_state.toggle_camera {
        camera_state.follow_enabled = !camera_state.follow_enabled;
    }
}

fn align_initial_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, With<PlayerEntity>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn follow_player(
    settings: Res<CameraSettings>,
    camera_state: Res<CameraState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, With<PlayerEntity>>,
    time: Res<Time>,
) {
    if !camera_state.follow_enabled {
        return;
    }

    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    let current = camera_transform.translation.truncate();
    let target = player_transform.translation.truncate();
    let delta = target - current;
    let offset = delta.clamp_length_max(settings.max_distance);

    let smoothing = 1.0 - (-settings.follow_damping * time.delta_seconds()).exp();
    let desired = current + offset * smoothing;
    let final_position = desired + camera_state.shake_offset;

    camera_transform.translation.x = final_position.x;
    camera_transform.translation.y = final_position.y;
}

fn apply_shake(
    settings: Res<CameraSettings>,
    mut camera_state: ResMut<CameraState>,
    time: Res<Time>,
) {
    if camera_state.desired_shake.length_squared() > 0.0 {
        camera_state.shake_offset = camera_state.desired_shake;
    }

    let decay = (1.0 - settings.shake_decay * time.delta_seconds()).clamp(0.0, 1.0);
    camera_state.shake_offset *= decay;
    camera_state.desired_shake *= decay;
}
