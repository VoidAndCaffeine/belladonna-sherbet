use avian3d::math::Scalar;
use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_tnua::builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::{TnuaAvian3dPlugin, TnuaAvian3dSensorShape};
use crate::plugins::camera::PlayerCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerSpawn;

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
enum ControlScheme {
    Jump(TnuaBuiltinJump),
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .register_type::<PlayerSpawn>()
            .add_observer(spawn_player)
            .add_systems(Update, apply_controls.in_set(TnuaUserControlsSystems))
            .add_plugins(TnuaControllerPlugin::<ControlScheme>::new(FixedUpdate))
            .add_plugins(TnuaAvian3dPlugin::new(FixedUpdate))
        ;
    }
}

fn spawn_player(
    _event: On<Add, PlayerSpawn>,
    query: Query<(&PlayerSpawn,&Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut control_scheme_configs: ResMut<Assets<ControlSchemeConfig>>,
) {
    let transform = query.single().unwrap().1;
    info!("Spawning player");
    let child
        = asset_server.load(GltfAssetLabel::Scene(0).from_asset("belladonna-sherbet.gltf"));
    commands.spawn((
        WorldAssetRoot(child),
        *transform,
        Player,
        RigidBody::Dynamic,
        TnuaController::<ControlScheme>::default(),
        TnuaConfig::<ControlScheme>(control_scheme_configs.add( ControlSchemeConfig {
            basis: TnuaBuiltinWalkConfig {
                float_height:1.0,
                ..Default::default()
            },
            jump: TnuaBuiltinJumpConfig {
                height: 4.0,
                ..Default::default()
            },
        })),
        TnuaAvian3dSensorShape(Collider::cylinder(0.49,0.0)),
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
    ));
    commands.spawn((
        PlayerCamera,
        Camera {
            order: 100,
            ..default()
        },
        AmbientLight{
            brightness:0.00,
            ..default()
        },
        Bloom::NATURAL,
    ));
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
    let jump = keyboard.any_pressed([KeyCode::Space]);

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

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if jump {
        controller.action(ControlScheme::Jump(Default::default()));
    }
}