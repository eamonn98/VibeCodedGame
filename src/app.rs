use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::window::WindowPlugin;

use crate::config;
use crate::core;
use crate::player;
use crate::rendering;
use crate::systems;
use crate::ui;
use crate::world;

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1280.0, 720.0).into(),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..Default::default()
                }),
        )
        .add_plugins(GamePlugin)
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            config::ConfigPlugin,
            core::CorePlugin,
            rendering::RenderingPlugin,
            world::WorldPlugin,
            player::PlayerPlugin,
            systems::SystemsPlugin,
            ui::UiPlugin,
        ));
    }
}
