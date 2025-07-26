use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::core_pipeline::auto_exposure::AutoExposure;
use bevy::core_pipeline::bloom::Bloom;
use bevy::pbr::Atmosphere;
use bevy::prelude::*;
use bevy::render::camera::{Exposure, PhysicalCameraParameters};
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController};
use bevy_tnua::TnuaUserControlsSystemSet;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use crate::plugins::camera::PlayerCamera;
use crate::plugins::input::InputAction;
use crate::prelude::asset_managment::LoadingData;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerSpawn;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .register_type::<PlayerSpawn>()
            .add_observer(spawn_player)
            .add_systems(FixedUpdate, apply_movement.in_set(TnuaUserControlsSystemSet))
        ;
    }
}
fn spawn_player(
    trigger: Trigger<OnAdd, PlayerSpawn>,
    query: Query<&Transform>,
    mut commands: Commands,
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>,
) {
    let transform = query.get(trigger.target()).unwrap();
    info!("spawning player");
    let child
        = asset_server.load(GltfAssetLabel::Scene(0).from_asset("belladonna-sherbet.gltf"));
    loading_data.loading_assets.push(child.clone().into());
    commands.spawn((
        SceneRoot(child),
        *transform,
        Player,
        RigidBody::Dynamic,
        TnuaController::default(),
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
        ));
    commands.spawn((
        PlayerCamera,
        Camera {
            hdr: true,
            ..default()
        },
        AmbientLight{
            brightness: 0.00,
            ..default()
        },
        Bloom::NATURAL,
        ));
}
fn apply_movement(
    mut input_events: EventReader<InputAction>,
    mut query: Query<&mut TnuaController>
) {
    let Ok(mut controller) = query.single_mut() else {
        return;
    };
    let mut direction = Vec3::ZERO;
    for event in input_events.read(){
        match event {
            InputAction::MoveAxis(axis) => {
                direction.x = axis.x;
                direction.z = axis.y;
            }
            _ => {}
        }
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * 10.0,
        desired_forward: Dir3::new(direction).ok(),
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 1.0,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });
}
