use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::systems::{CollisionLayers, PIXELS_PER_METER};
use crate::world::ChunkSettings;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_placeholder_player);
    }
}

#[derive(Component)]
pub struct PlayerEntity;

fn spawn_placeholder_player(
    mut commands: Commands,
    layers: Res<CollisionLayers>,
    chunk_settings: Res<ChunkSettings>,
) {
    let chunk_dims = chunk_settings.chunk_dimensions.as_vec2();
    let tile_size = chunk_settings.tile_size;
    let terrain_extent = chunk_dims * tile_size * 0.5;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.7, 0.95),
                custom_size: Some(Vec2::splat(48.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(terrain_extent.x, terrain_extent.y, 1.0),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Velocity::zero(),
        Damping {
            linear_damping: 12.0,
            angular_damping: 4.0,
        },
        Collider::ball(24.0 / PIXELS_PER_METER),
        Friction::coefficient(1.2),
        Restitution::coefficient(0.05),
        layers.player_groups(),
        LockedAxes::ROTATION_LOCKED,
        PlayerEntity,
        Name::new("Player"),
    ));
}
