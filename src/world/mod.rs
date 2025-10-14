use bevy::prelude::*;

pub mod generation;
mod tiles;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((generation::GenerationPlugin, tiles::TilesPlugin));
    }
}
