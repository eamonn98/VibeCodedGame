use bevy::prelude::*;

mod chunk;
pub mod generation;
mod tiles;

pub use chunk::{ChunkCoord, ChunkLayer, ChunkManifest, ChunkPlugin, ChunkSettings, ChunkTiles, WorldChunks};
pub use generation::TerrainSettings;
pub use tiles::{TileDefinition, TileId, TileRegistry, TilesPlugin};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            chunk::ChunkPlugin,
            tiles::TilesPlugin,
            generation::GenerationPlugin,
        ));
    }
}
