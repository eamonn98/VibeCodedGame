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
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.2, 0.7, 0.95),
                custom_size: Some(Vec2::splat(48.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
        PlayerEntity,
        Name::new("Player"),
    ));
}
