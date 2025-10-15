use bevy::prelude::*;
use std::collections::HashMap;

use crate::world::{ChunkSettings, TerrainSettings, TileRegistry, WorldChunks};

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ChunkDebugSprite {
    pub chunk: IVec2,
    pub local: UVec2,
}

#[derive(Resource)]
pub struct DebugTerrainSettings {
    pub show_tiles: bool,
}

impl Default for DebugTerrainSettings {
    fn default() -> Self {
        Self { show_tiles: true }
    }
}

#[derive(Resource, Default)]
pub struct ChunkDebugStats {
    pub frame_generated: u32,
    pub frame_updated: u32,
    pub frame_reused: u32,
    pub frame_unloaded: u32,
    pub total_generated: u64,
    pub total_unloaded: u64,
    pub overlay_toggles: u32,
    pub total_loaded: usize,
}

pub struct WorldDebugPlugin;

impl Plugin for WorldDebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugTerrainSettings>()
            .init_resource::<ChunkDebugStats>()
            .add_systems(PreUpdate, reset_chunk_debug_stats)
            .add_systems(Update, sync_chunk_debug_sprites);
    }
}

fn reset_chunk_debug_stats(mut stats: ResMut<ChunkDebugStats>) {
    stats.frame_generated = 0;
    stats.frame_updated = 0;
    stats.frame_reused = 0;
    stats.frame_unloaded = 0;
}

fn sync_chunk_debug_sprites(
    mut commands: Commands,
    debug_settings: Res<DebugTerrainSettings>,
    chunk_settings: Res<ChunkSettings>,
    terrain_settings: Res<TerrainSettings>,
    tiles: Res<TileRegistry>,
    chunks: Res<WorldChunks>,
    mut existing: Query<(
        Entity,
        &mut Transform,
        &mut Sprite,
        &mut Visibility,
        &mut Name,
        &ChunkDebugSprite,
    )>,
) {
    let mut desired: HashMap<ChunkDebugSprite, (Vec3, Color, String)> = HashMap::new();

    if debug_settings.show_tiles {
        let tile_size = chunk_settings.tile_size;
        let chunk_dims = chunk_settings.chunk_dimensions;

        for (coord, chunk) in chunks.iter() {
            let Some(layer) = chunk.layers.get(&terrain_settings.ground_layer) else {
                continue;
            };

            for y in 0..layer.size.y {
                for x in 0..layer.size.x {
                    let index = (y * layer.size.x + x) as usize;
                    let tile_id = layer.tiles[index];
                    let meta = ChunkDebugSprite {
                        chunk: coord.0,
                        local: UVec2::new(x, y),
                    };

                    let (color, label) = if let Some(def) = tiles.get(tile_id) {
                        let alpha = (0.3 + def.height * 0.25).clamp(0.2, 0.8);
                        let base = if def.walkable {
                            Color::srgba(0.3, 0.9, 0.3, alpha)
                        } else {
                            Color::srgba(0.9, 0.3, 0.3, alpha)
                        };
                        let label = format!("{} ({}, {})", def.name, coord.0.x, coord.0.y);
                        (base, label)
                    } else {
                        (
                            Color::srgba(0.2, 0.4, 0.9, 0.4),
                            format!("Tile {:?} ({}, {})", tile_id, coord.0.x, coord.0.y),
                        )
                    };

                    let pos = Vec3::new(
                        (coord.0.x * chunk_dims.x as i32 + x as i32) as f32 * tile_size.x
                            + tile_size.x * 0.5,
                        (coord.0.y * chunk_dims.y as i32 + y as i32) as f32 * tile_size.y
                            + tile_size.y * 0.5,
                        10.0,
                    );

                    desired.insert(meta, (pos, color, label));
                }
            }
        }
    }

    for (_, mut transform, mut sprite, mut visibility, mut name, meta) in existing.iter_mut() {
        if let Some((pos, color, label)) = desired.remove(meta) {
            transform.translation = pos;
            sprite.color = color;
            sprite.custom_size = Some(chunk_settings.tile_size);
            *visibility = Visibility::Visible;
            name.set(label);
        } else {
            *visibility = Visibility::Hidden;
        }
    }

    if !debug_settings.show_tiles {
        return;
    }

    let tile_size = chunk_settings.tile_size;

    for (meta, (pos, color, label)) in desired {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(tile_size),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos),
                visibility: Visibility::Visible,
                ..Default::default()
            },
            meta,
            Name::new(label),
        ));
    }
}
