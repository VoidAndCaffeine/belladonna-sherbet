use avian3d::{PhysicsPlugins};
use bevy::app::FixedUpdate;
use bevy::prelude::{App, Plugin, DefaultPlugins};

pub struct Default;
impl Plugin for Default {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                DefaultPlugins,
                PhysicsPlugins::default(),
            ));

    }
}