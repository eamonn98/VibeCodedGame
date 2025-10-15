use bevy::math::{IVec2, UVec2, Vec2};
use bevy::prelude::*;
use std::collections::HashMap;

use crate::world::tiles::TileId;

#[derive(Resource, Default)]
pub struct WorldChunks {
    pub loaded: HashMap<ChunkCoord, ChunkTiles>,
}

impl WorldChunks {
    pub fn ensure_chunk(&mut self, coord: ChunkCoord) -> &mut ChunkTiles {
        self.loaded.entry(coord).or_insert_with(ChunkTiles::new)
    }

    pub fn remove_chunk(&mut self, coord: &ChunkCoord) {
        self.loaded.remove(coord);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ChunkCoord, &ChunkTiles)> {
        self.loaded.iter()
    }

    pub fn tile_at_world(
        &self,
        settings: &ChunkSettings,
        layer: ChunkLayer,
        world_position: Vec2,
    ) -> Option<TileId> {
        let chunk_dims = settings.chunk_dimensions.as_ivec2();
        if chunk_dims.x == 0 || chunk_dims.y == 0 {
            return None;
        }

        let tile_coords = (world_position / settings.tile_size).floor().as_ivec2();
        let chunk_coord = ChunkCoord(tile_coords.div_euclid(chunk_dims));
        let local = tile_coords.rem_euclid(chunk_dims);

        let chunk = self.loaded.get(&chunk_coord)?;
        let layer_data = chunk.layers.get(&layer)?;
        layer_data.get(local.x as u32, local.y as u32)
    }
}

#[derive(Clone, Debug)]
pub struct ChunkLayerEntry {
    pub layer: ChunkLayer,
    pub name: String,
    pub z_index: f32,
}

#[derive(Resource, Default)]
pub struct ChunkManifest {
    pub layers: Vec<ChunkLayerEntry>,
}

impl ChunkManifest {
    pub fn register_layer(&mut self, name: impl Into<String>, z_index: f32) -> ChunkLayer {
        let layer = ChunkLayer(self.layers.len() as u8);
        self.layers.push(ChunkLayerEntry {
            layer,
            name: name.into(),
            z_index,
        });
        layer
    }

    pub fn layer_by_name(&self, name: &str) -> Option<ChunkLayer> {
        self.layers
            .iter()
            .find(|entry| entry.name == name)
            .map(|entry| entry.layer)
    }

    pub fn entries(&self) -> impl Iterator<Item = &ChunkLayerEntry> {
        self.layers.iter()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChunkCoord(pub IVec2);

#[derive(Clone, Debug)]
pub struct LayerData {
    pub tiles: Vec<TileId>,
    pub size: UVec2,
}

impl LayerData {
    pub fn new(size: UVec2) -> Self {
        Self {
            tiles: vec![TileId::default(); (size.x * size.y) as usize],
            size,
        }
    }

    fn index(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.size.x || y >= self.size.y {
            return None;
        }
        Some((y * self.size.x + x) as usize)
    }

    pub fn set(&mut self, x: u32, y: u32, tile: TileId) {
        if let Some(index) = self.index(x, y) {
            self.tiles[index] = tile;
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<TileId> {
        let index = self.index(x, y)?;
        Some(self.tiles[index])
    }

    pub fn fill(&mut self, tile: TileId) {
        self.tiles.fill(tile);
    }
}

#[derive(Clone, Debug)]
pub struct ChunkTiles {
    pub layers: HashMap<ChunkLayer, LayerData>,
    pub dirty: bool,
}

impl ChunkTiles {
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
            dirty: true,
        }
    }

    pub fn ensure_layer(&mut self, layer: ChunkLayer, size: UVec2) -> &mut LayerData {
        let entry = self
            .layers
            .entry(layer)
            .or_insert_with(|| LayerData::new(size));

        if entry.size != size {
            *entry = LayerData::new(size);
        }

        entry
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChunkLayer(pub u8);

#[derive(Resource, Debug)]
pub struct ChunkSettings {
    pub tile_size: Vec2,
    pub chunk_dimensions: UVec2,
}

impl Default for ChunkSettings {
    fn default() -> Self {
        Self {
            tile_size: Vec2::splat(48.0),
            chunk_dimensions: UVec2::new(32, 32),
        }
    }
}

impl ChunkSettings {
    pub fn chunk_pixel_size(&self) -> Vec2 {
        Vec2::new(
            self.chunk_dimensions.x as f32 * self.tile_size.x,
            self.chunk_dimensions.y as f32 * self.tile_size.y,
        )
    }
}

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkSettings>()
            .init_resource::<WorldChunks>()
            .init_resource::<ChunkManifest>();
    }
}
