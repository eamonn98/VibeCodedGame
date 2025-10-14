use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod settings;

pub use settings::AppConfig;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppConfig>()
            .add_systems(Startup, configure_window);
    }
}

fn configure_window(config: Res<AppConfig>, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.title = config.window_title.clone();
    }
}
