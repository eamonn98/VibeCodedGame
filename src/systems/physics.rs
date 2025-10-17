use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, render::DebugRenderContext};
use tracing::info;

pub const PIXELS_PER_METER: f32 = 100.0;

#[derive(Resource, Clone, Copy)]
pub struct CollisionLayers {
    pub player: Group,
    pub enemy: Group,
    pub terrain: Group,
}

impl Default for CollisionLayers {
    fn default() -> Self {
        Self {
            player: Group::GROUP_1,
            enemy: Group::GROUP_2,
            terrain: Group::GROUP_3,
        }
    }
}

impl CollisionLayers {
    pub fn player_groups(&self) -> CollisionGroups {
        CollisionGroups::new(self.player, self.enemy | self.terrain)
    }

    pub fn enemy_groups(&self) -> CollisionGroups {
        CollisionGroups::new(self.enemy, self.player | self.terrain)
    }

    pub fn terrain_groups(&self) -> CollisionGroups {
        CollisionGroups::new(self.terrain, self.player | self.enemy)
    }
}

pub struct PhysicsPlugin;

#[derive(Resource, Clone, Copy)]
pub struct PhysicsDebugSettings {
    pub show_colliders: bool,
}

impl Default for PhysicsDebugSettings {
    fn default() -> Self {
        Self {
            show_colliders: false,
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing Rapier physics");
        app.insert_resource(CollisionLayers::default())
            .insert_resource(PhysicsDebugSettings::default())
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(PreUpdate, apply_default_gravity)
            .add_systems(Update, sync_debug_render);
    }
}

fn apply_default_gravity(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}

fn sync_debug_render(
    settings: Res<PhysicsDebugSettings>,
    context: Option<ResMut<DebugRenderContext>>,
) {
    if let Some(mut context) = context {
        let desired = settings.show_colliders;
        if context.enabled != desired {
            context.enabled = desired;
        }
    }
}
