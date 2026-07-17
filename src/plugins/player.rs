use avian3d::math::Scalar;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::{TnuaAvian3dPlugin};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub enum ControlScheme {
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .add_systems(Update, apply_controls.in_set(TnuaUserControlsSystems))
            .add_plugins(TnuaControllerPlugin::<ControlScheme>::new(FixedUpdate))
            .add_plugins(TnuaAvian3dPlugin::new(FixedUpdate))
        ;
    }
}


fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TnuaController<ControlScheme>>,
){
    let Ok(mut controller) = query.single_mut() else {
        return;
    };

    controller.initiate_action_feeding();
    let up = keyboard.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let down = keyboard.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let left = keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let h = right as i8 - left as i8;
    let v = up as i8 - down as i8;
    let direction = Vec3::new(h as Scalar, 0.0, v as Scalar).clamp_length_max(1.0);
    // Set the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO` to reset the previous frame's input.
    controller.basis = TnuaBuiltinWalk {
        // The `desired_motion` determines how the character will move.
        desired_motion: direction.normalize_or_zero(),
        // The other field is `desired_forward` - but since the character model is a capsule we
        // don't care the direction its "forward" is pointing.
        ..Default::default()
    };
}