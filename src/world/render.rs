use bevy::log::info;
use bevy::prelude::*;
use std::collections::HashMap;

use crate::world::chunk::{ChunkCoord, ChunkSettings, WorldChunks};
use crate::world::generation::TerrainSettings;
use crate::world::tiles::{TileId, TileRegistry};

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct TerrainTileSprite {
    chunk: ChunkCoord,
    local: UVec2,
}

pub struct TerrainRenderPlugin;

impl Plugin for TerrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, sync_terrain_tile_sprites);
    }
}

fn sync_terrain_tile_sprites(
    mut commands: Commands,
    chunk_settings: Res<ChunkSettings>,
    terrain_settings: Res<TerrainSettings>,
    tiles: Res<TileRegistry>,
    chunks: Res<WorldChunks>,
    mut existing: Query<(
        Entity,
        &mut Transform,
        &mut Sprite,
        &mut Name,
        &TerrainTileSprite,
    )>,
) {
    if chunk_settings.chunk_dimensions.x == 0 || chunk_settings.chunk_dimensions.y == 0 {
        return;
    }

    let tile_size = chunk_settings.tile_size;
    if tile_size.x <= 0.0 || tile_size.y <= 0.0 {
        return;
    }

    let mut desired: HashMap<TerrainTileSprite, (TileId, Vec3, Color, String)> = HashMap::new();

    let chunk_dims = chunk_settings.chunk_dimensions;
    let chunk_dims_f = chunk_dims.as_vec2();
    let tile_size_vec2 = tile_size;
    let half_tile = tile_size_vec2 * 0.5;

    for (&coord, chunk) in chunks.iter() {
        let Some(layer) = chunk.layers.get(&terrain_settings.ground_layer) else {
            continue;
        };

        for y in 0..layer.size.y {
            for x in 0..layer.size.x {
                let index = (y * layer.size.x + x) as usize;
                let tile_id = layer.tiles[index];
                let meta = TerrainTileSprite {
                    chunk: coord,
                    local: UVec2::new(x, y),
                };

                let (color, height, label) = match tiles.get(tile_id) {
                    Some(def) => {
                        let base_color = if def.walkable {
                            Color::srgba(0.38, 0.78, 0.42, 1.0)
                        } else {
                            Color::srgba(0.85, 0.32, 0.32, 1.0)
                        };
                        let label = format!("{} ({}, {})", def.name, coord.0.x, coord.0.y);
                        (base_color, def.height, label)
                    }
                    None => (
                        Color::srgba(0.25, 0.25, 0.35, 1.0),
                        0.0,
                        format!("Tile {:?} ({}, {})", tile_id, coord.0.x, coord.0.y),
                    ),
                };

                let chunk_origin_tiles = coord.0.as_vec2() * chunk_dims_f;
                let chunk_origin_world = chunk_origin_tiles * tile_size_vec2;
                let cell_offset = Vec2::new(x as f32, y as f32) * tile_size_vec2;
                let world_pos = chunk_origin_world + cell_offset + half_tile;
                let translation = Vec3::new(world_pos.x, world_pos.y, height.max(0.0));

                desired.insert(meta, (tile_id, translation, color, label));
            }
        }
    }

    for (entity, mut transform, mut sprite, mut name, meta) in existing.iter_mut() {
        if let Some((_, translation, color, label)) = desired.remove(meta) {
            transform.translation = translation;
            sprite.color = color;
            sprite.custom_size = Some(tile_size_vec2);
            name.set(label);
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }

    let mut spawned = 0usize;

    for (meta, (_tile_id, translation, color, label)) in desired {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(tile_size_vec2),
                    ..Default::default()
                },
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
            meta,
            Name::new(format!("{} [{},{}]", label, meta.local.x, meta.local.y)),
        ));
        spawned += 1;
    }

    if spawned > 0 {
        info!(
            "Spawned {} terrain sprites (total chunks: {})",
            spawned,
            chunks.loaded.len()
        );
    }
}
