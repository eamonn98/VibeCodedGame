use bevy::asset::AssetPlugin;
use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy::window::WindowPlugin;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use crate::config;
use crate::core;
use crate::enemy;
use crate::player;
use crate::rendering;
use crate::systems;
use crate::ui;
use crate::world;

pub fn run() {
    init_tracing();
    info!("Starting VibeCoded Game app");
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
            AudioPlugin::default(),
            config::ConfigPlugin,
            core::CorePlugin,
            rendering::RenderingPlugin,
            world::WorldPlugin,
            enemy::EnemyPlugin,
            player::PlayerPlugin,
            systems::SystemsPlugin,
            ui::UiPlugin,
        ));
    }
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let _ = fmt().with_env_filter(filter).with_target(false).try_init();
}
