use bevy::core_pipeline::bloom::BloomSettings;
use bevy::math::primitives::Rectangle;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle};

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PostProcessingSettings>()
            .register_type::<PostProcessingSettings>()
            .add_plugins(Material2dPlugin::<VignetteMaterial>::default())
            .add_systems(Startup, setup_vignette_overlay)
            .add_systems(
                Update,
                (
                    apply_bloom_settings,
                    sync_vignette_overlay,
                    apply_vignette_settings,
                ),
            );
    }
}

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct PostProcessingSettings {
    pub bloom_enabled: bool,
    pub bloom_intensity: f32,
    pub bloom_threshold: f32,
    pub vignette_enabled: bool,
    pub vignette_intensity: f32,
    pub vignette_power: f32,
}

impl Default for PostProcessingSettings {
    fn default() -> Self {
        Self {
            bloom_enabled: true,
            bloom_intensity: 0.35,
            bloom_threshold: -1.0,
            vignette_enabled: true,
            vignette_intensity: 0.35,
            vignette_power: 1.6,
        }
    }
}

#[derive(Component)]
struct VignetteOverlay;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct VignetteMaterial {
    #[uniform(0)]
    params: VignetteParams,
}

impl Default for VignetteMaterial {
    fn default() -> Self {
        Self {
            params: VignetteParams::default(),
        }
    }
}

impl Material2d for VignetteMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path("shaders/vignette_material.wgsl".into())
    }

    fn vertex_shader() -> ShaderRef {
        ShaderRef::Path("shaders/vignette_material.wgsl".into())
    }
}

#[derive(Clone, Copy, Default, ShaderType)]
struct VignetteParams {
    intensity: f32,
    power: f32,
}

fn setup_vignette_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<VignetteMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(Rectangle::new(2.0, 2.0)));
    let material = materials.add(VignetteMaterial::default());

    commands.spawn((
        MaterialMesh2dBundle::<VignetteMaterial> {
            mesh: Mesh2dHandle(mesh),
            material,
            transform: Transform::from_xyz(0.0, 0.0, 950.0),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        VignetteOverlay,
        Name::new("Vignette Overlay"),
    ));
}

fn apply_bloom_settings(
    settings: Res<PostProcessingSettings>,
    mut query: Query<(&mut BloomSettings, &mut Camera)>,
) {
    if !settings.is_changed() {
        return;
    }

    for (mut bloom, mut camera) in &mut query {
        camera.hdr = settings.bloom_enabled;

        bloom.intensity = if settings.bloom_enabled {
            settings.bloom_intensity
        } else {
            0.0
        };
        bloom.prefilter_settings.threshold = settings.bloom_threshold;
    }
}

fn sync_vignette_overlay(
    camera_query: Query<(&GlobalTransform, &Camera, Option<&OrthographicProjection>)>,
    mut overlay_query: Query<&mut Transform, With<VignetteOverlay>>,
) {
    let Ok((camera_transform, camera, projection)) = camera_query.get_single() else {
        return;
    };
    let Ok(mut transform) = overlay_query.get_single_mut() else {
        return;
    };

    let position = camera_transform.translation();
    transform.translation.x = position.x;
    transform.translation.y = position.y;

    if let Some(ortho) = projection {
        let size = ortho.area.size();
        transform.scale = Vec3::new(size.x * 1.2, size.y * 1.2, 1.0);
    } else if let Some(size) = camera.logical_viewport_size() {
        transform.scale = Vec3::new(size.x * 1.2, size.y * 1.2, 1.0);
    }
}

fn apply_vignette_settings(
    settings: Res<PostProcessingSettings>,
    mut materials: ResMut<Assets<VignetteMaterial>>,
    mut overlay_query: Query<(&Handle<VignetteMaterial>, &mut Visibility), With<VignetteOverlay>>,
) {
    if overlay_query.is_empty() {
        return;
    }

    let visible = settings.vignette_enabled && settings.vignette_intensity > 0.0;

    for (handle, mut visibility) in &mut overlay_query {
        *visibility = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        if let Some(material) = materials.get_mut(handle) {
            material.params.intensity = settings.vignette_intensity;
            material.params.power = settings.vignette_power;
        }
    }
}
