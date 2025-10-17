use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod debug_overlay;
mod egui_panel;

use crate::world::debug::WorldDebugPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin,
            debug_overlay::DebugOverlayPlugin,
            egui_panel::DebugPanelPlugin,
            WorldDebugPlugin,
        ));
    }
}
