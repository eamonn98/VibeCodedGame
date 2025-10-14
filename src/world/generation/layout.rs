use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ChunkDimensions {
    pub width: u32,
    pub height: u32,
}

pub fn register(app: &mut App) {
    app.init_resource::<ChunkDimensions>();
}
