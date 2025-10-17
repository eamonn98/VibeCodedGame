use bevy::log::info;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::HashSet;

use crate::player::PlayerEntity;
use crate::world::chunk::{
    ChunkCoord, ChunkLayer, ChunkManifest, ChunkSettings, ChunkTiles, WorldChunks,
};
use crate::world::debug::ChunkDebugStats;
use crate::world::generation::{NoiseSettings, WorldSeed};
use crate::world::tiles::TileId;

#[derive(Resource)]
pub struct TerrainSettings {
    pub ground_layer: ChunkLayer,
    pub walkable_tile: TileId,
    pub blocking_tile: TileId,
    pub view_radius: i32,
    pub noise_threshold: f64,
}

impl Default for TerrainSettings {
    fn default() -> Self {
        Self {
            ground_layer: ChunkLayer(0),
            walkable_tile: TileId(0),
            blocking_tile: TileId(1),
            view_radius: 2,
            noise_threshold: 0.1,
        }
    }
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainSettings>()
            .add_systems(Startup, setup_chunk_layers)
            .add_systems(Update, generate_visible_chunks);
    }
}

fn setup_chunk_layers(
    mut manifest: ResMut<ChunkManifest>,
    mut terrain_settings: ResMut<TerrainSettings>,
) {
    if manifest.layers.is_empty() {
        terrain_settings.ground_layer = manifest.register_layer("ground", 0.0);
    }
}

fn generate_visible_chunks(
    mut chunks: ResMut<WorldChunks>,
    settings: Res<ChunkSettings>,
    terrain_settings: Res<TerrainSettings>,
    noise_settings: Res<NoiseSettings>,
    seed: Res<WorldSeed>,
    player_query: Query<&Transform, With<PlayerEntity>>,
    mut stats: ResMut<ChunkDebugStats>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let perlin = Perlin::new(seed.0 as u32);

    let chunk_size = settings.chunk_dimensions;
    if chunk_size.x == 0 || chunk_size.y == 0 {
        return;
    }

    let chunk_dims_i = chunk_size.as_ivec2();
    let tile_size = settings.tile_size;
    if tile_size.x.abs() < f32::EPSILON || tile_size.y.abs() < f32::EPSILON {
        return;
    }

    let tile_coords = (player_transform.translation.truncate() / tile_size)
        .floor()
        .as_ivec2();
    let player_chunk = ChunkCoord(tile_coords.div_euclid(chunk_dims_i));

    let sample_scale = noise_settings.scale.max(1.0) as f64;
    let persistence = noise_settings.persistence.max(0.01) as f64;
    let lacunarity = noise_settings.lacunarity.max(1.0) as f64;
    let view_radius = terrain_settings.view_radius.max(1);

    let mut desired: HashSet<ChunkCoord> = HashSet::new();

    let mut generated = 0u32;
    let mut updated = 0u32;
    let mut reused = 0u32;

    for dx in -view_radius..=view_radius {
        for dy in -view_radius..=view_radius {
            let coord = ChunkCoord(player_chunk.0 + IVec2::new(dx, dy));
            desired.insert(coord);

            if let Some(chunk_tiles) = chunks.loaded.get_mut(&coord) {
                if chunk_tiles.dirty {
                    generate_chunk_tiles(
                        chunk_tiles,
                        coord,
                        chunk_dims_i,
                        chunk_size,
                        terrain_settings.ground_layer,
                        terrain_settings.walkable_tile,
                        terrain_settings.blocking_tile,
                        &perlin,
                        sample_scale,
                        persistence,
                        lacunarity,
                        terrain_settings.noise_threshold,
                    );
                    chunk_tiles.mark_clean();
                    updated += 1;
                } else {
                    reused += 1;
                }
            } else {
                let mut chunk_tiles = ChunkTiles::new();
                generate_chunk_tiles(
                    &mut chunk_tiles,
                    coord,
                    chunk_dims_i,
                    chunk_size,
                    terrain_settings.ground_layer,
                    terrain_settings.walkable_tile,
                    terrain_settings.blocking_tile,
                    &perlin,
                    sample_scale,
                    persistence,
                    lacunarity,
                    terrain_settings.noise_threshold,
                );
                chunk_tiles.mark_clean();
                chunks.loaded.insert(coord, chunk_tiles);
                generated += 1;
            }
        }
    }

    let mut unloaded = 0u32;
    let loaded_keys: Vec<ChunkCoord> = chunks.loaded.keys().copied().collect();
    for coord in loaded_keys {
        if !desired.contains(&coord) {
            chunks.loaded.remove(&coord);
            unloaded += 1;
        }
    }

    stats.frame_generated = generated;
    stats.frame_updated = updated;
    stats.frame_reused = reused;
    stats.frame_unloaded = unloaded;
    stats.total_generated += generated as u64;
    stats.total_unloaded += unloaded as u64;
    stats.total_loaded = chunks.loaded.len();

    if generated > 0 || updated > 0 || unloaded > 0 {
        info!(
            "Chunks -> frame g:{} upd:{} reused:{} un:{} | total load:{} gen:{} un:{}",
            generated,
            updated,
            reused,
            unloaded,
            stats.total_loaded,
            stats.total_generated,
            stats.total_unloaded
        );
    }
}

fn generate_chunk_tiles(
    chunk_tiles: &mut ChunkTiles,
    coord: ChunkCoord,
    chunk_dims_i: IVec2,
    chunk_size: UVec2,
    layer_id: ChunkLayer,
    walkable_tile: TileId,
    blocking_tile: TileId,
    perlin: &Perlin,
    sample_scale: f64,
    persistence: f64,
    lacunarity: f64,
    threshold: f64,
) {
    let layer = chunk_tiles.ensure_layer(layer_id, chunk_size);
    layer.fill(walkable_tile);

    let chunk_origin = coord.0 * chunk_dims_i;

    for y in 0..chunk_size.y {
        for x in 0..chunk_size.x {
            let cell = chunk_origin + IVec2::new(x as i32, y as i32);
            let mut amplitude = 1.0;
            let mut frequency = 1.0;
            let mut amplitude_sum = 0.0;
            let mut noise_sum = 0.0;

            for _ in 0..4 {
                let sample_x = cell.x as f64 / sample_scale * frequency;
                let sample_y = cell.y as f64 / sample_scale * frequency;
                let value = perlin.get([sample_x, sample_y]);
                noise_sum += value * amplitude;
                amplitude_sum += amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;
            }

            let noise_value = if amplitude_sum > 0.0 {
                noise_sum / amplitude_sum
            } else {
                0.0
            };

            if noise_value > threshold {
                layer.set(x, y, blocking_tile);
            }
        }
    }
}
