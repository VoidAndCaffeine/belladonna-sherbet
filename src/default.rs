use avian3d::{PhysicsPlugins};
use bevy::app::FixedUpdate;
use bevy::prelude::{App, Plugin, DefaultPlugins};
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

pub struct Default;
impl Plugin for Default {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                DefaultPlugins,
                PhysicsPlugins::default(),
                TnuaControllerPlugin::new(FixedUpdate),
                TnuaAvian3dPlugin::new(FixedUpdate)
            ));

    }
}