use bevy::prelude::*;

use crate::systems::movement::{MovementState, MovementSystemSet};
use crate::world::{ChunkSettings, TerrainSettings, TileRegistry, WorldChunks};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_tile_collisions.after(MovementSystemSet::Update));
    }
}

fn apply_tile_collisions(
    mut query: Query<(&mut Transform, &mut MovementState)>,
    chunks: Res<WorldChunks>,
    chunk_settings: Res<ChunkSettings>,
    terrain_settings: Res<TerrainSettings>,
    tiles: Res<TileRegistry>,
) {
    if chunk_settings.chunk_dimensions.x == 0 || chunk_settings.chunk_dimensions.y == 0 {
        return;
    }

    let tile_size = chunk_settings.tile_size;
    if tile_size.x.abs() < f32::EPSILON || tile_size.y.abs() < f32::EPSILON {
        return;
    }

    for (mut transform, mut movement) in &mut query {
        if movement.desired_translation == Vec2::ZERO {
            continue;
        }

        let proposed = transform.translation.truncate() + movement.desired_translation;

        let collider = chunks.tile_at_world(&chunk_settings, terrain_settings.ground_layer, proposed);

        if let Some(tile_id) = collider {
            if !tiles.is_walkable(tile_id) {
                movement.velocity = Vec2::ZERO;
                movement.desired_translation = Vec2::ZERO;
                continue;
            }
        }

        transform.translation.x = proposed.x;
        transform.translation.y = proposed.y;
        movement.desired_translation = Vec2::ZERO;
    }
}
