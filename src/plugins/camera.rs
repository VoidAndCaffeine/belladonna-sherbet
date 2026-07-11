use bevy::prelude::*;
use crate::plugins::game::GameState;
use crate::plugins::player::Player;

const CAMERA_DISTANCE:f32 = 15.0;
const CAMERA_VECTOR:Vec3= Vec3::new(0.0,1.0,1.0);

#[derive(Component,Reflect)]
#[reflect(Component)]
#[require(Camera3d)]
pub(crate) struct PlayerCamera;

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerCamera>()
            .add_systems(Update, update_camera_pos
                .run_if(in_state(GameState::InGame))
            )
        ;
    }
}

fn update_camera_pos(
    player_transform: Single<&Transform, With<Player>>,
    mut camera_transform: Single<&mut Transform,(With<PlayerCamera>, Without<Player>)>,
) {
    camera_transform.translation =
        player_transform.translation + CAMERA_DISTANCE * CAMERA_VECTOR;
    camera_transform.look_at(player_transform.translation, Vec3::Y);
}