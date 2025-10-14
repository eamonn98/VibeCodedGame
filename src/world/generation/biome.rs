use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct BiomeRegistry {
    pub entries: Vec<String>,
}

pub fn register(app: &mut App) {
    app.init_resource::<BiomeRegistry>();
}
