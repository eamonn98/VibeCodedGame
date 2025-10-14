use bevy::prelude::*;

use crate::config::AppConfig;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .insert_resource(CameraState::default());
    }
}

#[derive(Resource, Default)]
pub struct CameraState {
    pub follow_enabled: bool,
}

fn setup_camera(mut commands: Commands, config: Res<AppConfig>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 0,
                ..Default::default()
            },
            ..Default::default()
        },
        Name::new(format!("{} Camera", config.window_title)),
    ));
}
