use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, render::ColliderDebugColor};
use std::collections::HashMap;

use crate::systems::{CollisionLayers, PIXELS_PER_METER};
use crate::world::chunk::{ChunkCoord, ChunkSettings, LayerData, WorldChunks};
use crate::world::generation::TerrainSettings;
use crate::world::tiles::TileRegistry;

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TerrainCollider {
    pub chunk: ChunkCoord,
}

pub struct WorldPhysicsPlugin;

impl Plugin for WorldPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_chunk_colliders);
    }
}

fn sync_chunk_colliders(
    mut commands: Commands,
    chunk_settings: Res<ChunkSettings>,
    terrain_settings: Res<TerrainSettings>,
    tile_registry: Res<TileRegistry>,
    collision_layers: Res<CollisionLayers>,
    chunks: Res<WorldChunks>,
    existing: Query<(Entity, &TerrainCollider)>,
) {
    let mut existing_map: HashMap<ChunkCoord, Entity> = existing
        .iter()
        .map(|(entity, meta)| (meta.chunk, entity))
        .collect();

    let chunk_dims = chunk_settings.chunk_dimensions;
    let tile_size = chunk_settings.tile_size;
    if tile_size.x <= 0.0 || tile_size.y <= 0.0 {
        return;
    }

    for (&coord, chunk) in chunks.loaded.iter() {
        let Some(layer) = chunk.layers.get(&terrain_settings.ground_layer) else {
            continue;
        };

        let collider = build_chunk_collider(layer, tile_size, &tile_registry);

        match (existing_map.remove(&coord), collider) {
            (Some(entity), coll_opt) => update_chunk_collider(
                &mut commands,
                entity,
                coord,
                coll_opt,
                tile_size,
                chunk_dims,
            ),
            (None, Some(collider)) => spawn_chunk_collider(
                &mut commands,
                coord,
                collider,
                tile_size,
                chunk_dims,
                &collision_layers,
            ),
            (None, None) => {}
        }
    }

    for (_, entity) in existing_map {
        commands.entity(entity).despawn_recursive();
    }
}

fn build_chunk_collider(
    layer: &LayerData,
    tile_size: Vec2,
    tiles: &TileRegistry,
) -> Option<Collider> {
    let mut shapes: Vec<(Vect, Rot, Collider)> = Vec::new();

    let max_x = layer.size.x;
    let max_y = layer.size.y;

    for y in 0..max_y {
        let mut span_start: Option<u32> = None;

        for x in 0..=max_x {
            let blocking = if x < max_x {
                let idx = (y * max_x + x) as usize;
                let tile_id = layer.tiles[idx];
                !tiles.is_walkable(tile_id)
            } else {
                false
            };

            match (span_start, blocking) {
                (None, true) => span_start = Some(x),
                (Some(start), false) => {
                    append_span(start, x, y, tile_size, &mut shapes);
                    span_start = None;
                }
                _ => {}
            }
        }
    }

    if shapes.is_empty() {
        None
    } else {
        Some(Collider::compound(shapes))
    }
}

fn append_span(
    start_x: u32,
    end_x: u32,
    y: u32,
    tile_size: Vec2,
    shapes: &mut Vec<(Vect, Rot, Collider)>,
) {
    let width_tiles = end_x - start_x;
    if width_tiles == 0 {
        return;
    }

    let width_pixels = width_tiles as f32 * tile_size.x;
    let height_pixels = tile_size.y;

    let half_extents = Vec2::new(
        width_pixels * 0.5 / PIXELS_PER_METER,
        height_pixels * 0.5 / PIXELS_PER_METER,
    );

    let center_x_pixels = (start_x as f32 * tile_size.x) + width_pixels * 0.5;
    let center_y_pixels = (y as f32 * tile_size.y) + tile_size.y * 0.5;

    let translation = Vect::new(
        center_x_pixels / PIXELS_PER_METER,
        center_y_pixels / PIXELS_PER_METER,
    );

    shapes.push((
        translation,
        0.0,
        Collider::cuboid(half_extents.x, half_extents.y),
    ));
}

fn spawn_chunk_collider(
    commands: &mut Commands,
    coord: ChunkCoord,
    collider: Collider,
    tile_size: Vec2,
    chunk_dims: UVec2,
    collision_layers: &CollisionLayers,
) {
    let translation = chunk_translation(coord, tile_size, chunk_dims);
    commands.spawn((
        RigidBody::Fixed,
        collider,
        collision_layers.terrain_groups(),
        TerrainCollider { chunk: coord },
        ColliderDebugColor(Hsla::new(30.0, 0.8, 0.6, 0.4)),
        TransformBundle::from_transform(Transform::from_translation(translation)),
        Name::new(format!("Terrain Collider ({}, {})", coord.0.x, coord.0.y)),
    ));
}

fn update_chunk_collider(
    commands: &mut Commands,
    entity: Entity,
    coord: ChunkCoord,
    collider: Option<Collider>,
    tile_size: Vec2,
    chunk_dims: UVec2,
) {
    if let Some(collider) = collider {
        let translation = chunk_translation(coord, tile_size, chunk_dims);
        commands.entity(entity).insert((
            collider,
            TransformBundle::from_transform(Transform::from_translation(translation)),
        ));
    } else {
        commands.entity(entity).despawn_recursive();
    }
}

fn chunk_translation(coord: ChunkCoord, tile_size: Vec2, chunk_dims: UVec2) -> Vec3 {
    let origin = coord.0 * chunk_dims.as_ivec2();
    Vec3::new(
        origin.x as f32 * tile_size.x,
        origin.y as f32 * tile_size.y,
        0.0,
    )
}
