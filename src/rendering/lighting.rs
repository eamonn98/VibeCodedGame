use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::sprite::Material2dPlugin;

pub const MAX_POINT_LIGHTS: usize = 4;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LightingSettings>()
            .register_type::<LightingSettings>()
            .register_type::<DirectionalLight2d>()
            .register_type::<PointLight2d>()
            .add_plugins(Material2dPlugin::<Hd2dLightingMaterial>::default())
            .add_systems(Startup, setup_default_directional_light)
            .add_systems(Update, sync_lighting_materials);
    }
}

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct LightingSettings {
    pub ambient_color: Color,
    pub ambient_intensity: f32,
    pub normal_strength: f32,
}

impl Default for LightingSettings {
    fn default() -> Self {
        Self {
            ambient_color: Color::srgb(0.15, 0.18, 0.24),
            ambient_intensity: 0.6,
            normal_strength: 1.0,
        }
    }
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct DirectionalLight2d {
    pub direction: Vec2,
    pub color: Color,
    pub intensity: f32,
}

impl Default for DirectionalLight2d {
    fn default() -> Self {
        Self {
            direction: Vec2::new(-0.45, -0.9).normalize_or_zero(),
            color: Color::srgb(1.0, 0.96, 0.87),
            intensity: 1.25,
        }
    }
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct PointLight2d {
    pub radius: f32,
    pub color: Color,
    pub intensity: f32,
}

impl Default for PointLight2d {
    fn default() -> Self {
        Self {
            radius: 320.0,
            color: Color::srgb(0.96, 0.82, 0.62),
            intensity: 1.0,
        }
    }
}

fn setup_default_directional_light(mut commands: Commands) {
    commands.spawn((DirectionalLight2d::default(), Name::new("Sunlight")));
}

#[derive(Clone, Copy, ShaderType)]
pub struct Hd2dMaterialParams {
    pub base_color: Vec4,
    pub ambient_color: Vec4,
    pub directional_color: Vec4,
    pub directional_dir: Vec2,
    pub normal_strength: f32,
    pub point_light_count: u32,
    pub point_positions: [Vec4; MAX_POINT_LIGHTS],
    pub point_colors: [Vec4; MAX_POINT_LIGHTS],
}

impl Default for Hd2dMaterialParams {
    fn default() -> Self {
        Self {
            base_color: Vec4::ONE,
            ambient_color: Vec4::ZERO,
            directional_color: Vec4::ZERO,
            directional_dir: Vec2::ZERO,
            normal_strength: 1.0,
            point_light_count: 0,
            point_positions: [Vec4::ZERO; MAX_POINT_LIGHTS],
            point_colors: [Vec4::ZERO; MAX_POINT_LIGHTS],
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct Hd2dLightingMaterial {
    #[uniform(0)]
    pub params: Hd2dMaterialParams,
}

impl Default for Hd2dLightingMaterial {
    fn default() -> Self {
        Self {
            params: Hd2dMaterialParams::default(),
        }
    }
}

impl bevy::sprite::Material2d for Hd2dLightingMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path("shaders/hd2d_material.wgsl".into())
    }
}

fn sync_lighting_materials(
    settings: Res<LightingSettings>,
    mut materials: ResMut<Assets<Hd2dLightingMaterial>>,
    directional_lights: Query<&DirectionalLight2d>,
    point_lights: Query<(&PointLight2d, &GlobalTransform)>,
) {
    if materials.is_empty() {
        return;
    }

    let primary_directional = directional_lights
        .iter()
        .next()
        .cloned()
        .unwrap_or_default();

    let ambient_linear = settings.ambient_color.to_linear();
    let ambient_color = Vec4::new(
        ambient_linear.red * settings.ambient_intensity,
        ambient_linear.green * settings.ambient_intensity,
        ambient_linear.blue * settings.ambient_intensity,
        settings.ambient_intensity,
    );

    let dir_linear = primary_directional.color.to_linear();
    let directional_color = Vec4::new(
        dir_linear.red * primary_directional.intensity,
        dir_linear.green * primary_directional.intensity,
        dir_linear.blue * primary_directional.intensity,
        primary_directional.intensity,
    );

    let mut point_positions = [Vec4::ZERO; MAX_POINT_LIGHTS];
    let mut point_colors = [Vec4::ZERO; MAX_POINT_LIGHTS];
    let mut point_count = 0u32;

    for (light, transform) in point_lights.iter().take(MAX_POINT_LIGHTS) {
        let linear = light.color.to_linear();
        let pos = transform.translation().truncate();
        point_positions[point_count as usize] = Vec4::new(pos.x, pos.y, light.radius, 0.0);
        point_colors[point_count as usize] = Vec4::new(
            linear.red * light.intensity,
            linear.green * light.intensity,
            linear.blue * light.intensity,
            light.intensity,
        );
        point_count += 1;
    }

    for (_, material) in materials.iter_mut() {
        material.params.base_color = Vec4::ONE;
        material.params.ambient_color = ambient_color;
        material.params.directional_color = directional_color;
        material.params.directional_dir = primary_directional.direction;
        material.params.normal_strength = settings.normal_strength;
        material.params.point_light_count = point_count;
        material.params.point_positions = point_positions;
        material.params.point_colors = point_colors;
    }
}
