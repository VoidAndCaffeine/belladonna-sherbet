#![allow(unused_imports)]
use bevy::app::{App, Plugin};

mod plugins;
mod default;

mod prelude{
    pub use super::{plugins::*,};
}

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                default::Default,
                plugins::GamePlugins,
            ));

        // this is only needed to send components to blender,
        // thus it is not needed in release builds
        #[cfg(feature = "dev")]
        app.add_plugins(plugins::dev::DevPlugins);
    }
}