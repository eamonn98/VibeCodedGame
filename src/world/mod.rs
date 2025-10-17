use bevy::prelude::*;

mod chunk;
pub mod debug;
pub mod generation;
mod physics;
mod render;
mod tiles;

pub use chunk::{ChunkSettings, WorldChunks};
pub use generation::TerrainSettings;
pub use physics::TerrainCollider;
pub use tiles::TileRegistry;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            chunk::ChunkPlugin,
            tiles::TilesPlugin,
            generation::GenerationPlugin,
            physics::WorldPhysicsPlugin,
            render::TerrainRenderPlugin,
        ));
    }
}
