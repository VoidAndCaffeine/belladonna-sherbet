use avian3d::math::{Scalar, Vector2};
use bevy::prelude::*;

pub(crate) struct InputManagerPlugin;
impl Plugin for InputManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<InputAction>()
            .add_systems(Update, keyboard_input)
        ;
    }
}

#[derive(Event)]
pub enum InputAction{
    MoveAxis(Vec2),
    CamAxis(Vec2)
}

fn keyboard_input(
    mut movement_event_writer: EventWriter<InputAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    // TODO: camera inputs
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let h = right as i8 - left as i8;
    let v = up as i8 - down as i8;

    let direction = Vector2::new(h as Scalar, v as Scalar).clamp_length_max(1.0);
    if direction != Vec2::ZERO {
        movement_event_writer.write(InputAction::MoveAxis(direction));
    }
}