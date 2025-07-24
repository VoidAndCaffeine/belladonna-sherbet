use bevy::prelude::*;
use avian3d::{prelude::*, math::*};
use bevy::input::InputPlugin;
use crate::plugins::player_controler::movement;

pub struct InputManagerPlugin;
impl Plugin for InputManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MovementAction>()
            .add_systems(Update,(keyboard_input,gamepad_input).before(movement))
        ;
    }
}

/// movement input event
#[derive(Event)]
pub enum MovementAction {
    Move(Vector2),
}


/// Sends [`MovementAction`] events based on keyboard input.
//TODO: rewrite
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vector2::new(horizontal as Scalar, vertical as Scalar).clamp_length_max(1.0);

    if direction != Vector2::ZERO {
        movement_event_writer.write(MovementAction::Move(direction));
    }
}

/// Sends [`MovementAction`] events based on gamepad input.
//TODO: rewrite
fn gamepad_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    gamepads: Query<&Gamepad>,
) {
    for gamepad in gamepads.iter() {
        if let (Some(x), Some(y)) = (
            gamepad.get(GamepadAxis::LeftStickX),
            gamepad.get(GamepadAxis::LeftStickY),
        ) {
            movement_event_writer.write(MovementAction::Move(
                Vector2::new(x as Scalar, y as Scalar).clamp_length_max(1.0),
            ));
        }
    }
}