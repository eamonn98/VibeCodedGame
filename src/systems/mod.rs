use bevy::prelude::*;

mod collisions;
mod movement;
mod physics;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            movement::MovementPlugin,
            physics::PhysicsPlugin,
            collisions::CollisionPlugin,
        ));
    }
}
