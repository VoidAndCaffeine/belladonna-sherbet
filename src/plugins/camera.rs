use bevy::prelude::*;
use crate::plugins::game::GameState;
use crate::plugins::player::Player;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum CameraState{
    Static,
    #[default]
    Dynamic,
}
const CAMERA_DISTANCE:f32 = 15.0;
const CAMERA_VECTOR:Vec3= Vec3::new(0.0,1.0,1.0);

#[derive(Component,Reflect)]
#[reflect(Component)]
pub(crate) struct PlayerCamera;

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerCamera>()
            .init_state::<CameraState>()
            .add_systems(Update, update_camera_pos
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(CameraState::Dynamic))
            )
        ;
    }
}

fn update_camera_pos(
    camera_state: Res<State<CameraState>>,
    mut player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform,( With<Camera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single_mut()
    else {
        error!("Error getting global transform");
        return;
    };

    let Ok(mut camera_transform) = camera_query.single_mut()
    else {
        error!("No Cameras Found");
        return;
    };

    camera_transform.translation
        = player_transform.translation + CAMERA_DISTANCE * CAMERA_VECTOR;
    camera_transform.look_at(player_transform.translation,Vec3::Y);

}