use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::core::{CameraSettings, CameraState, TimeScale};
use crate::systems::PhysicsDebugSettings;

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
) {
    if !state.open {
        return;
    }

    let ctx = contexts.ctx_mut();
    egui::Window::new("Debug Controls")
        .resizable(true)
        .default_size((280.0, 200.0))
        .show(ctx, |ui| {
            ui.heading("Simulation");
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

            ui.separator();
            ui.heading("Physics");
            let mut show_colliders = physics_debug.show_colliders;
            if ui.checkbox(&mut show_colliders, "Show Colliders").changed() {
                physics_debug.show_colliders = show_colliders;
            }

            if ui.button("Close").clicked() {
                state.open = false;
            }
        });
}
