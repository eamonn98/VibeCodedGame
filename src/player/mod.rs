use bevy::prelude::*;

mod animation;
mod controller;

pub use controller::PlayerEntity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((controller::ControllerPlugin, animation::AnimationPlugin));
    }
}
