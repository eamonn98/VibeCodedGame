use bevy::prelude::*;

use crate::core::TimeScale;
use crate::player::PlayerEntity;
use crate::systems::MovementState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dummy_enemy)
            .add_systems(Update, enemy_chase_player);
    }
}

#[derive(Component)]
pub struct EnemyEntity;

fn spawn_dummy_enemy(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.9, 0.7, 0.2, 1.0),
                custom_size: Some(Vec2::splat(48.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(192.0, 96.0, 1.0),
            ..Default::default()
        },
        MovementState::default(),
        EnemyEntity,
        Name::new("Enemy Dummy"),
    ));
}

fn enemy_chase_player(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    player_query: Query<&Transform, With<PlayerEntity>>,
    mut enemies: Query<(&Transform, &mut MovementState), With<EnemyEntity>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let dt = (time.delta_seconds() * time_scale.0).max(f32::EPSILON);
    let target = player_transform.translation.truncate();

    for (transform, mut movement) in enemies.iter_mut() {
        let to_player = target - transform.translation.truncate();
        if to_player.length_squared() < 1.0 {
            movement.desired_translation = Vec2::ZERO;
            movement.velocity = Vec2::ZERO;
            continue;
        }

        let speed = 80.0;
        let desired = to_player.normalize() * speed * dt;
        movement.desired_translation = desired;
        movement.velocity = desired / dt;
        movement.blocked = false;
    }
}
