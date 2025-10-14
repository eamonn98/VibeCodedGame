use bevy::prelude::*;

mod debug_overlay;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug_overlay::DebugOverlayPlugin);
    }
}
