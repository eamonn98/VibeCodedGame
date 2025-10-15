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
    pub dash_pressed: bool,
    pub dash_just_pressed: bool,
    pub attack: bool,
    pub toggle_camera: bool,
    pub toggle_debug_tiles: bool,
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

    let dash_pressed = keyboard.pressed(KeyCode::Space);
    let dash_just_pressed = keyboard.just_pressed(KeyCode::Space);

    input_state.movement = movement.normalize_or_zero();
    input_state.dash_pressed = dash_pressed;
    input_state.dash_just_pressed = dash_just_pressed;
    input_state.attack = keyboard.just_pressed(KeyCode::KeyJ);
    input_state.toggle_camera = keyboard.just_pressed(KeyCode::KeyC);
    input_state.toggle_debug_tiles = keyboard.just_pressed(KeyCode::KeyV);
}
