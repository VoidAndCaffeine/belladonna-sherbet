#![allow(unused_imports)]
use bevy::app::{App, Plugin};

mod plugins;
mod default;

mod prelude{
    pub use super::*;
    pub use {plugins::*,};
}

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                default::Default,
                plugins::GamePlugins,
            ));
    }
}