use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TileId(pub u32);

#[derive(Clone, Debug)]
pub struct TileDefinition {
    pub id: TileId,
    pub name: String,
    pub walkable: bool,
    pub height: f32,
}

#[derive(Resource, Default)]
pub struct TileRegistry {
    pub entries: HashMap<TileId, TileDefinition>,
}

impl TileRegistry {
    pub fn register(&mut self, definition: TileDefinition) {
        self.entries.insert(definition.id, definition);
    }

    pub fn get(&self, id: TileId) -> Option<&TileDefinition> {
        self.entries.get(&id)
    }

    pub fn is_walkable(&self, id: TileId) -> bool {
        self.get(id).map(|def| def.walkable).unwrap_or(true)
    }
}

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileRegistry>()
            .add_systems(Startup, register_default_tiles);
    }
}

fn register_default_tiles(mut registry: ResMut<TileRegistry>) {
    if registry.entries.is_empty() {
        registry.register(TileDefinition {
            id: TileId(0),
            name: "Empty".into(),
            walkable: true,
            height: 0.0,
        });

        registry.register(TileDefinition {
            id: TileId(1),
            name: "Wall".into(),
            walkable: false,
            height: 1.0,
        });
    }
}
