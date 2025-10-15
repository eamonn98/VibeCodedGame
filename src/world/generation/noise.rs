use bevy::prelude::*;

#[derive(Resource)]
pub struct NoiseSettings {
    pub scale: f32,
    pub persistence: f32,
    pub lacunarity: f32,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            scale: 48.0,
            persistence: 0.5,
            lacunarity: 2.0,
        }
    }
}

pub fn register(app: &mut App) {
    app.init_resource::<NoiseSettings>();
}
