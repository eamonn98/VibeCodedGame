use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, RapierContext, Velocity};

use crate::systems::{MovementState, PIXELS_PER_METER};
use crate::world::TerrainCollider;

#[derive(Resource, Default)]
struct MovementCollisionTracker {
    counts: HashMap<Entity, u32>,
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementCollisionTracker>().add_systems(
            PostUpdate,
            (handle_collision_events, apply_collision_corrections).chain(),
        );
    }
}

fn handle_collision_events(
    mut events: EventReader<CollisionEvent>,
    mut tracker: ResMut<MovementCollisionTracker>,
    mut movers: Query<(&mut MovementState, Option<&mut Velocity>)>,
    terrain: Query<(), With<TerrainCollider>>,
) {
    for event in events.read() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                process_collision(*a, *b, true, &mut tracker, &mut movers, &terrain);
                process_collision(*b, *a, true, &mut tracker, &mut movers, &terrain);
            }
            CollisionEvent::Stopped(a, b, _) => {
                process_collision(*a, *b, false, &mut tracker, &mut movers, &terrain);
                process_collision(*b, *a, false, &mut tracker, &mut movers, &terrain);
            }
        }
    }
}

fn process_collision(
    mover: Entity,
    other: Entity,
    started: bool,
    tracker: &mut MovementCollisionTracker,
    movers: &mut Query<(&mut MovementState, Option<&mut Velocity>)>,
    terrain: &Query<(), With<TerrainCollider>>,
) {
    if terrain.get(other).is_err() {
        return;
    }

    let Ok((mut movement, velocity)) = movers.get_mut(mover) else {
        return;
    };

    if started {
        let count = tracker.counts.entry(mover).or_insert(0);
        *count += 1;
        movement.blocked = true;
        movement.velocity = Vec2::ZERO;
        movement.desired_translation = Vec2::ZERO;
        if let Some(mut velocity) = velocity {
            velocity.linvel = Vec2::ZERO;
        }
    } else if let Some(count) = tracker.counts.get_mut(&mover) {
        if *count > 0 {
            *count -= 1;
        }
        if *count == 0 {
            tracker.counts.remove(&mover);
            movement.blocked = false;
        }
    }
}

fn apply_collision_corrections(
    tracker: Res<MovementCollisionTracker>,
    rapier_context: Res<RapierContext>,
    terrain: Query<(), With<TerrainCollider>>,
    mut movers: Query<&mut Transform>,
) {
    let mut corrections: HashMap<Entity, (Vec2, u32)> = HashMap::new();

    for (&entity, &count) in tracker.counts.iter() {
        if count == 0 {
            continue;
        }

        for pair in rapier_context.contact_pairs_with(entity) {
            let mover_is_first = pair.collider1() == entity;
            let other = if mover_is_first {
                pair.collider2()
            } else {
                pair.collider1()
            };

            if terrain.get(other).is_err() {
                continue;
            }

            if let Some((manifold, contact)) = pair.find_deepest_contact() {
                let dist = contact.dist();
                if dist < 0.0 {
                    let mut normal: Vec2 = manifold.normal().into();
                    if !mover_is_first {
                        normal = -normal;
                    }

                    let depth = (-dist).max(0.0);
                    let entry = corrections.entry(entity).or_insert((Vec2::ZERO, 0));
                    entry.0 += normal * (depth + 0.01);
                    entry.1 += 1;
                }
            }
        }
    }

    for (entity, (sum, count)) in corrections {
        if count == 0 {
            continue;
        }

        let correction = (sum / count as f32) * PIXELS_PER_METER;

        if let Ok(mut transform) = movers.get_mut(entity) {
            transform.translation.x += correction.x;
            transform.translation.y += correction.y;
        }
    }
}
