use bevy::prelude::*;

mod collisions;
mod combat;
mod movement;
mod physics;

pub use combat::{ComboTracker, EnemyDeathEvent, EnemyHitEvent};
pub use movement::{MovementState, MovementSystemSet};

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
