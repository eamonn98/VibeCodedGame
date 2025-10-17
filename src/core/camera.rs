use bevy::prelude::*;

use crate::config::AppConfig;
use crate::core::InputState;
use crate::player::PlayerEntity;
use crate::world::ChunkSettings;

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

fn setup_camera(
    mut commands: Commands,
    config: Res<AppConfig>,
    chunk_settings: Res<ChunkSettings>,
) {
    let chunk_dims = chunk_settings.chunk_dimensions.as_vec2();
    let tile_size = chunk_settings.tile_size;
    let terrain_extent = chunk_dims * tile_size * 0.5;

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
            transform: Transform::from_xyz(terrain_extent.x, terrain_extent.y, 999.9),
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
    mut queries: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<PlayerEntity>>,
    )>,
) {
    let player_translation = {
        let player_query = queries.p1();
        match player_query.get_single() {
            Ok(transform) => transform.translation,
            Err(_) => return,
        }
    };

    let mut camera_query = queries.p0();
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    camera_transform.translation.x = player_translation.x;
    camera_transform.translation.y = player_translation.y;
}

fn follow_player(
    settings: Res<CameraSettings>,
    camera_state: Res<CameraState>,
    mut queries: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<PlayerEntity>>,
    )>,
    time: Res<Time>,
) {
    if !camera_state.follow_enabled {
        return;
    }

    let player_translation = {
        let player_query = queries.p1();
        match player_query.get_single() {
            Ok(transform) => transform.translation,
            Err(_) => return,
        }
    };

    let mut camera_query = queries.p0();
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    let current = camera_transform.translation.truncate();
    let target = player_translation.truncate();
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
