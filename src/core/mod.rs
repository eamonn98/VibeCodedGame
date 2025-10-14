use bevy::prelude::*;

mod camera;
mod input;
mod time;

pub use camera::CameraState;
pub use input::InputState;
pub use time::TimeScale;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, input::InputPlugin, time::TimePlugin));
    }
}
