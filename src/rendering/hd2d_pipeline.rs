use bevy::prelude::*;
use bevy::transform::TransformSystem;

#[derive(Resource)]
pub struct Hd2dSettings {
    pub tilt_radians: f32,
    pub depth_scale: f32,
}

impl Default for Hd2dSettings {
    fn default() -> Self {
        Self {
            tilt_radians: 0.35,
            depth_scale: 0.0015,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct BillboardSprite {
    pub tilt_override: Option<f32>,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct DepthSorted {
    pub base_z: f32,
    pub depth_scale: Option<f32>,
}

impl DepthSorted {
    pub fn new(base_z: f32, depth_scale: f32) -> Self {
        Self {
            base_z,
            depth_scale: Some(depth_scale),
        }
    }
}

pub struct Hd2dPipelinePlugin;

impl Plugin for Hd2dPipelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Hd2dSettings>().add_systems(
            PostUpdate,
            (apply_depth_sorting, apply_billboard_sprites)
                .chain()
                .after(TransformSystem::TransformPropagate),
        );
    }
}

fn apply_depth_sorting(
    settings: Res<Hd2dSettings>,
    mut query: Query<(&DepthSorted, &mut Transform)>,
) {
    if query.is_empty() {
        return;
    }

    let default_scale = settings.depth_scale;
    for (depth, mut transform) in &mut query {
        let scale = depth.depth_scale.unwrap_or(default_scale);
        transform.translation.z = depth.base_z - transform.translation.y * scale;
    }
}

fn apply_billboard_sprites(
    settings: Res<Hd2dSettings>,
    mut query: Query<(&BillboardSprite, &mut Transform)>,
) {
    if query.is_empty() {
        return;
    }

    let default_rotation = Quat::from_rotation_x(settings.tilt_radians);
    for (billboard, mut transform) in &mut query {
        let rotation = billboard
            .tilt_override
            .map(Quat::from_rotation_x)
            .unwrap_or(default_rotation);
        transform.rotation = rotation;
    }
}
