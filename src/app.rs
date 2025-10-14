use crate::config;
use crate::core;
use crate::player;
use crate::rendering;
use crate::systems;
use crate::ui;
use crate::world;
use bevy::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
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
