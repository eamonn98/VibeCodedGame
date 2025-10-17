use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::core::{CameraSettings, CameraState, TimeScale};
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
                    .show(ui, draw_rendering_tab);

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

fn draw_rendering_tab(ui: &mut egui::Ui) {
    ui.heading("Camera Effects");
    ui.label("Post-processing controls are placeholders.");
    let mut bloom = 0.0_f32;
    ui.add_enabled(
        false,
        egui::Slider::new(&mut bloom, 0.0..=1.0).text("Bloom"),
    );
    let mut vignette = 0.0_f32;
    ui.add_enabled(
        false,
        egui::Slider::new(&mut vignette, 0.0..=1.0).text("Vignette"),
    );
    let mut aberration = 0.0_f32;
    ui.add_enabled(
        false,
        egui::Slider::new(&mut aberration, 0.0..=1.0).text("Chromatic Aberration"),
    );
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
