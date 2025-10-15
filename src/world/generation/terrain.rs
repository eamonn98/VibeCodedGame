use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};

use crate::world::chunk::{ChunkCoord, ChunkLayer, ChunkManifest, ChunkSettings, WorldChunks};
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

fn setup_chunk_layers(mut manifest: ResMut<ChunkManifest>, mut terrain_settings: ResMut<TerrainSettings>) {
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
    player_query: Query<&Transform>,
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
    let view_radius = terrain_settings.view_radius.max(1);

    for dx in -view_radius..=view_radius {
        for dy in -view_radius..=view_radius {
            let coord = ChunkCoord(player_chunk.0 + IVec2::new(dx, dy));
            let chunk_tiles = chunks.ensure_chunk(coord);
            if !chunk_tiles.dirty {
                continue;
            }

            let layer = chunk_tiles.ensure_layer(terrain_settings.ground_layer, chunk_size);
            layer.fill(terrain_settings.walkable_tile);

            let chunk_origin = coord.0 * chunk_dims_i;

            for y in 0..chunk_size.y {
                for x in 0..chunk_size.x {
                    let cell = chunk_origin + IVec2::new(x as i32, y as i32);
                    let noise_value = perlin.get([
                        cell.x as f64 / sample_scale,
                        cell.y as f64 / sample_scale,
                    ]);

                    if noise_value > terrain_settings.noise_threshold {
                        layer.set(x, y, terrain_settings.blocking_tile);
                    }
                }
            }

            chunk_tiles.mark_clean();
        }
    }
}
