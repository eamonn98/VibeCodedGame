use bevy::prelude::*;
use std::ops::RangeInclusive;

use crate::world::tiles::TileId;

#[derive(Clone, Debug)]
pub struct BiomeDefinition {
    pub name: String,
    pub noise_range: RangeInclusive<f64>,
    pub tint: Color,
    pub tile_id: TileId,
}

#[derive(Resource, Default)]
pub struct BiomeRegistry {
    pub entries: Vec<BiomeDefinition>,
}

impl BiomeRegistry {
    pub fn register(&mut self, definition: BiomeDefinition) {
        self.entries.push(definition);
    }

    pub fn by_name(&self, name: &str) -> Option<&BiomeDefinition> {
        self.entries.iter().find(|entry| entry.name == name)
    }
}

pub fn register(app: &mut App) {
    app.init_resource::<BiomeRegistry>()
        .add_systems(Startup, register_default_biomes);
}

fn register_default_biomes(mut registry: ResMut<BiomeRegistry>) {
    if !registry.entries.is_empty() {
        return;
    }

    registry.register(BiomeDefinition {
        name: "Grassland".into(),
        noise_range: (-1.0)..=0.15,
        tint: Color::srgba(0.38, 0.78, 0.42, 1.0),
        tile_id: TileId(0),
    });

    registry.register(BiomeDefinition {
        name: "Forest".into(),
        noise_range: 0.15..=0.45,
        tint: Color::srgba(0.18, 0.52, 0.28, 1.0),
        tile_id: TileId(0),
    });

    registry.register(BiomeDefinition {
        name: "Mountain".into(),
        noise_range: 0.45..=1.0,
        tint: Color::srgba(0.58, 0.52, 0.48, 1.0),
        tile_id: TileId(1),
    });
}
