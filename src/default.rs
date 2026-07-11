use avian3d::{PhysicsPlugins};
use bevy::app::FixedUpdate;
use bevy::prelude::{Color, App, Plugin, DefaultPlugins, ClearColor};
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;

pub struct Default;
impl Plugin for Default {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::srgb(0.0,0.0,0.0)))
            .add_plugins((
                DefaultPlugins,
                PhysicsPlugins::default(),
                TnuaControllerPlugin::new(FixedUpdate),
                TnuaAvian3dPlugin::new(FixedUpdate)
            ));

    }
}