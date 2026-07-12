use bevy::prelude::*;

#[derive(Component,Reflect)]
#[reflect(Component)]
#[require(Camera3d)]
pub(crate) struct PlayerCamera;

pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerCamera>()
            .add_systems(Startup, setup)
        ;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 1.0).looking_to(Vec3::ZERO, Vec3::Y),
        TransformGizmoCamera,
    ));
}