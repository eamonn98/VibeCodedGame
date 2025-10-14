use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct NoiseSettings {
    pub scale: f32,
    pub persistence: f32,
    pub lacunarity: f32,
}

pub fn register(app: &mut App) {
    app.init_resource::<NoiseSettings>();
}
