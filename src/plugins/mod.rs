use bevy::app::{App, Plugin};

pub mod player;
mod dev;
pub mod scene_builder;

pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(player::PlayerPlugin)
            .add_plugins(scene_builder::SceneBuilderPlugin)
        ;

        // this is only needed to send components to blender,
        // thus it is not needed in release builds
        #[cfg(feature = "dev")]
        app.add_plugins(dev::DevPlugins);
    }
}