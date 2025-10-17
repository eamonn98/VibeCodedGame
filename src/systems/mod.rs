use bevy::prelude::*;

mod collisions;
mod combat;
mod movement;
mod physics;

pub use physics::{CollisionLayers, PhysicsDebugSettings, PIXELS_PER_METER};

pub use combat::{ComboTracker, EnemyDeathEvent, EnemyHitEvent};
pub use movement::MovementState;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            movement::MovementPlugin,
            physics::PhysicsPlugin,
            collisions::CollisionPlugin,
            combat::CombatPlugin,
        ));
    }
}
