use bevy::input::ButtonInput;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_systems(Update, map_inputs);
    }
}

#[derive(Resource, Default)]
pub struct InputState {
    pub movement: Vec2,
    pub dash: bool,
    pub attack: bool,
}

fn map_inputs(mut input_state: ResMut<InputState>, keyboard: Res<ButtonInput<KeyCode>>) {
    let mut movement = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }

    input_state.movement = movement.normalize_or_zero();
    input_state.dash = keyboard.just_pressed(KeyCode::Space);
    input_state.attack = keyboard.just_pressed(KeyCode::KeyJ);
}
