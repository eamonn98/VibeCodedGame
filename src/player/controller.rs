use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_placeholder_player);
    }
}

#[derive(Component)]
pub struct PlayerEntity;

fn spawn_placeholder_player(mut commands: Commands) {
    commands.spawn((SpriteBundle::default(), PlayerEntity, Name::new("Player")));
}
