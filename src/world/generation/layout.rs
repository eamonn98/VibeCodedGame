use bevy::prelude::*;

use crate::world::chunk::ChunkSettings;

#[derive(Resource)]
pub struct ChunkDimensions {
    pub width: u32,
    pub height: u32,
}

impl Default for ChunkDimensions {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
        }
    }
}

pub fn register(app: &mut App) {
    app.init_resource::<ChunkDimensions>()
        .add_systems(Startup, apply_chunk_dimensions);
}

fn apply_chunk_dimensions(dims: Res<ChunkDimensions>, mut settings: ResMut<ChunkSettings>) {
    if settings.chunk_dimensions.x != dims.width || settings.chunk_dimensions.y != dims.height {
        settings.chunk_dimensions = UVec2::new(dims.width, dims.height);
    }
}
