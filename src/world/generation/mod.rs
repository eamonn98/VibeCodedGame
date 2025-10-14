use bevy::prelude::*;

mod biome;
mod layout;
mod noise;

pub use biome::BiomeRegistry;
pub use layout::ChunkDimensions;
pub use noise::NoiseSettings;

#[derive(Resource, Default)]
pub struct WorldSeed(pub u64);

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldSeed>();
        biome::register(app);
        layout::register(app);
        noise::register(app);
    }
}
