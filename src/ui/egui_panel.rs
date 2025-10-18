use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::core::{CameraSettings, CameraState, TimeScale};
use crate::rendering::lighting::{DirectionalLight2d, LightingSettings};
use crate::rendering::post_processing::PostProcessingSettings;
use crate::systems::PhysicsDebugSettings;
use crate::world::generation::{NoiseSettings, TerrainSettings};

pub struct DebugPanelPlugin;

impl Plugin for DebugPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugPanelState>()
            .add_systems(Update, (toggle_panel, draw_panel));
    }
}

#[derive(Resource)]
struct DebugPanelState {
    open: bool,
}

impl Default for DebugPanelState {
    fn default() -> Self {
        Self { open: false }
    }
}

fn toggle_panel(mut state: ResMut<DebugPanelState>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::F3) {
        state.open = !state.open;
    }
}

fn draw_panel(
    mut contexts: EguiContexts,
    mut state: ResMut<DebugPanelState>,
    mut time_scale: ResMut<TimeScale>,
    mut camera_settings: ResMut<CameraSettings>,
    mut camera_state: ResMut<CameraState>,
    mut physics_debug: ResMut<PhysicsDebugSettings>,
    mut terrain_settings: ResMut<TerrainSettings>,
    mut noise_settings: ResMut<NoiseSettings>,
    mut lighting_settings: ResMut<LightingSettings>,
    mut directional_lights: Query<&mut DirectionalLight2d>,
    mut post_settings: ResMut<PostProcessingSettings>,
) {
    if !state.open {
        return;
    }

    let ctx = contexts.ctx_mut();
    egui::Window::new("Debug Controls")
        .resizable(true)
        .default_size((360.0, 320.0))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::CollapsingHeader::new("Simulation")
                    .default_open(true)
                    .show(ui, |ui| {
                        draw_simulation_tab(
                            ui,
                            &mut time_scale,
                            &mut camera_settings,
                            &mut camera_state,
                        );
                    });

                egui::CollapsingHeader::new("Physics")
                    .default_open(true)
                    .show(ui, |ui| {
                        draw_physics_tab(ui, &mut physics_debug);
                    });

                egui::CollapsingHeader::new("Rendering")
                    .default_open(false)
                    .show(ui, |ui| {
                        let mut primary_light = directional_lights.iter_mut().next();
                        let primary_light_mut = primary_light.as_deref_mut();
                        draw_rendering_tab(
                            ui,
                            &mut lighting_settings,
                            primary_light_mut,
                            &mut post_settings,
                        );
                    });

                egui::CollapsingHeader::new("World Gen")
                    .default_open(false)
                    .show(ui, |ui| {
                        draw_world_tab(ui, &mut terrain_settings, &mut noise_settings);
                    });

                ui.separator();
                if ui.button("Close").clicked() {
                    state.open = false;
                }
            });
        });
}

fn draw_simulation_tab(
    ui: &mut egui::Ui,
    time_scale: &mut ResMut<TimeScale>,
    camera_settings: &mut ResMut<CameraSettings>,
    camera_state: &mut ResMut<CameraState>,
) {
    ui.heading("Time");
    let mut scale = time_scale.0;
    if ui
        .add(egui::Slider::new(&mut scale, 0.1..=2.0).text("Time Scale"))
        .changed()
    {
        time_scale.0 = scale;
    }

    ui.separator();
    ui.heading("Camera");
    let mut follow = camera_state.follow_enabled;
    if ui.checkbox(&mut follow, "Follow Player").changed() {
        camera_state.follow_enabled = follow;
    }

    let mut damping = camera_settings.follow_damping;
    if ui
        .add(egui::Slider::new(&mut damping, 1.0..=20.0).text("Follow Damping"))
        .changed()
    {
        camera_settings.follow_damping = damping;
    }

    let mut max_distance = camera_settings.max_distance;
    if ui
        .add(egui::Slider::new(&mut max_distance, 50.0..=600.0).text("Max Distance"))
        .changed()
    {
        camera_settings.max_distance = max_distance;
    }

    let mut shake_decay = camera_settings.shake_decay;
    if ui
        .add(egui::Slider::new(&mut shake_decay, 0.5..=10.0).text("Shake Decay"))
        .changed()
    {
        camera_settings.shake_decay = shake_decay;
    }

    if ui.button("Reset Shake").clicked() {
        camera_state.shake_offset = Vec2::ZERO;
        camera_state.desired_shake = Vec2::ZERO;
    }
}

fn draw_physics_tab(ui: &mut egui::Ui, physics_debug: &mut ResMut<PhysicsDebugSettings>) {
    ui.heading("Diagnostics");
    let mut show_colliders = physics_debug.show_colliders;
    if ui.checkbox(&mut show_colliders, "Show Colliders").changed() {
        physics_debug.show_colliders = show_colliders;
    }
}

fn draw_rendering_tab(
    ui: &mut egui::Ui,
    lighting_settings: &mut LightingSettings,
    mut primary_light: Option<&mut DirectionalLight2d>,
    post_settings: &mut PostProcessingSettings,
) {
    ui.heading("Lighting");

    let mut ambient_intensity = lighting_settings.ambient_intensity;
    if ui
        .add(egui::Slider::new(&mut ambient_intensity, 0.0..=1.5).text("Ambient Intensity"))
        .changed()
    {
        lighting_settings.ambient_intensity = ambient_intensity;
    }

    let mut normal_strength = lighting_settings.normal_strength;
    if ui
        .add(egui::Slider::new(&mut normal_strength, 0.0..=2.5).text("Normal Strength"))
        .changed()
    {
        lighting_settings.normal_strength = normal_strength;
    }

    if let Some(light) = primary_light.as_deref_mut() {
        ui.separator();
        ui.label("Directional Light");

        let mut intensity = light.intensity;
        if ui
            .add(egui::Slider::new(&mut intensity, 0.0..=3.0).text("Intensity"))
            .changed()
        {
            light.intensity = intensity;
        }

        let mut angle = light.direction.y.atan2(light.direction.x);
        if ui
            .add(egui::Slider::new(&mut angle, -PI..=PI).text("Direction"))
            .changed()
        {
            let dir = Vec2::new(angle.cos(), angle.sin()).normalize_or_zero();
            light.direction = if dir == Vec2::ZERO {
                light.direction
            } else {
                dir
            };
        }
    } else {
        ui.separator();
        ui.label("No directional light available");
    }

    ui.separator();
    ui.heading("Post Processing");

    let mut bloom_enabled = post_settings.bloom_enabled;
    if ui.checkbox(&mut bloom_enabled, "Bloom").changed() {
        post_settings.bloom_enabled = bloom_enabled;
    }

    ui.add_enabled_ui(post_settings.bloom_enabled, |ui| {
        let mut bloom_intensity = post_settings.bloom_intensity;
        if ui
            .add(egui::Slider::new(&mut bloom_intensity, 0.0..=2.0).text("Bloom Intensity"))
            .changed()
        {
            post_settings.bloom_intensity = bloom_intensity;
        }

        let mut bloom_threshold = post_settings.bloom_threshold;
        if ui
            .add(egui::Slider::new(&mut bloom_threshold, -5.0..=5.0).text("Bloom Threshold"))
            .changed()
        {
            post_settings.bloom_threshold = bloom_threshold;
        }
    });

    ui.separator();
    let mut vignette_enabled = post_settings.vignette_enabled;
    if ui.checkbox(&mut vignette_enabled, "Vignette").changed() {
        post_settings.vignette_enabled = vignette_enabled;
    }

    ui.add_enabled_ui(post_settings.vignette_enabled, |ui| {
        let mut intensity = post_settings.vignette_intensity;
        if ui
            .add(egui::Slider::new(&mut intensity, 0.0..=1.0).text("Vignette Intensity"))
            .changed()
        {
            post_settings.vignette_intensity = intensity;
        }

        let mut power = post_settings.vignette_power;
        if ui
            .add(egui::Slider::new(&mut power, 0.5..=4.0).text("Vignette Power"))
            .changed()
        {
            post_settings.vignette_power = power;
        }
    });
}

fn draw_world_tab(
    ui: &mut egui::Ui,
    terrain_settings: &mut ResMut<TerrainSettings>,
    noise_settings: &mut ResMut<NoiseSettings>,
) {
    ui.heading("Terrain View");
    let mut radius = terrain_settings.view_radius as f32;
    if ui
        .add(egui::Slider::new(&mut radius, 1.0..=6.0).text("Chunk Radius"))
        .changed()
    {
        terrain_settings.view_radius = radius.round() as i32;
    }

    let mut threshold = terrain_settings.noise_threshold as f32;
    if ui
        .add(egui::Slider::new(&mut threshold, -0.5..=0.5).text("Noise Threshold"))
        .changed()
    {
        terrain_settings.noise_threshold = threshold as f64;
    }

    ui.separator();
    ui.heading("Noise");
    let mut scale = noise_settings.scale;
    if ui
        .add(egui::Slider::new(&mut scale, 16.0..=96.0).text("Scale"))
        .changed()
    {
        noise_settings.scale = scale;
    }

    let mut persistence = noise_settings.persistence;
    if ui
        .add(egui::Slider::new(&mut persistence, 0.1..=0.9).text("Persistence"))
        .changed()
    {
        noise_settings.persistence = persistence;
    }

    let mut lacunarity = noise_settings.lacunarity;
    if ui
        .add(egui::Slider::new(&mut lacunarity, 1.0..=4.0).text("Lacunarity"))
        .changed()
    {
        noise_settings.lacunarity = lacunarity;
    }
}
