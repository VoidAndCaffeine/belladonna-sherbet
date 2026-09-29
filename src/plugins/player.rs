use avian3d::math::Scalar;
use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use bevy_tnua::builtins::TnuaBuiltinWalkConfig;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::{TnuaAvian3dPlugin, TnuaAvian3dSensorShape};
use serde::{Deserialize, Serialize};
use crate::plugins::camera::PlayerCamera;
use crate::plugins::game::{DataPath, GameState};
use crate::plugins::yarn::DialogueState;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub enum ControlScheme{}

#[derive(Resource,Serialize,Deserialize)]
pub struct KeyboardKeyBindings {
    pub up: [KeyCode; 2],
    pub down: [KeyCode; 2],
    pub left: [KeyCode; 2],
    pub right: [KeyCode; 2],
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerSpawn;
#[derive(States, Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum PlayerSpawnState {
    #[default]
    NotYetSpawned,
    Respawn,
    Spawned,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .register_type::<PlayerSpawn>()
            .init_state::<PlayerSpawnState>()
            .add_systems(Startup,setup_keybindings)
            .add_systems(Update, apply_controls
                .in_set(TnuaUserControlsSystems)
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(DialogueState::Game))
            )
            .add_plugins(TnuaControllerPlugin::<ControlScheme>::new(FixedUpdate))
            .add_plugins(TnuaAvian3dPlugin::new(FixedUpdate))
            .add_observer(spawn_player.run_if(in_state(PlayerSpawnState::NotYetSpawned)))
        ;
    }
}

fn setup_keybindings(
    mut commands: Commands,
    data_path: Res<DataPath>
) {
    let config_dir = data_path.path.join("Config");
    commands.insert_resource(
        Persistent::<KeyboardKeyBindings>::builder()
            .name("keyboard key bindings")
            .format(StorageFormat::TomlPretty)
            .path(config_dir.join("keyboard-keybindings.toml"))
            .default(
                KeyboardKeyBindings{
                    up: [KeyCode::KeyW, KeyCode::ArrowUp],
                    down: [KeyCode::KeyS, KeyCode::ArrowDown],
                    left: [KeyCode::KeyA, KeyCode::ArrowLeft],
                    right: [KeyCode::KeyD, KeyCode::ArrowRight],
                }
            )
            .revertible(true)
            .revert_to_default_on_deserialization_errors(true)
            .build()
            .expect("failed to init keyboard keybindings")
    )
}

fn spawn_player(
    _event: On<Add, PlayerSpawn>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut control_scheme_configs: ResMut<Assets<ControlSchemeConfig>>,
    query: Query<&Transform, With<PlayerSpawn>>
){
    let transform = match query.single() {
        Ok(transform) => transform,
        Err(_) => return,
    };
    let child
        = asset_server.load(GltfAssetLabel::Scene(0).from_asset("belladonna-sherbet.gltf"));
    commands.spawn((
        WorldAssetRoot(child),
        Transform::from_translation(transform.translation + Vec3::new(0.0, 0.8, 0.0)),
        Player,
        RigidBody::Dynamic,
        TnuaController::<ControlScheme>::default(),
        TnuaConfig::<ControlScheme>(control_scheme_configs.add(ControlSchemeConfig {
            basis: TnuaBuiltinWalkConfig {
                float_height:0.8,
                ..Default::default()
            }
        })),
        TnuaAvian3dSensorShape(Collider::cylinder((0.4 * 0.75),0.0)),
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
        Collider::capsule((0.5 * 0.75),0.8),
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

pub fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TnuaController<ControlScheme>>,
    kb_bindings: Res<Persistent<KeyboardKeyBindings>>,
){
    let Ok(mut controller) = query.single_mut() else {
        return;
    };

    controller.initiate_action_feeding();
    let up = keyboard.any_pressed(kb_bindings.up);
    let down = keyboard.any_pressed(kb_bindings.down);
    let left = keyboard.any_pressed(kb_bindings.left);
    let right = keyboard.any_pressed(kb_bindings.right);

    let h = right as i8 - left as i8;
    let v = down as i8 - up as i8;
    let direction = Vec3::new(h as Scalar, 0.0, v as Scalar).clamp_length_max(1.0).normalize_or_zero();
    // Set the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO` to reset the previous frame's input.
    controller.basis = TnuaBuiltinWalk {
        // The `desired_motion` determines how the character will move.
        desired_motion: direction,
        desired_forward: Dir3::new(direction).ok(),
        ..Default::default()
    };
}