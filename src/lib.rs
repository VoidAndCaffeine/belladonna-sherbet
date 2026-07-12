use bevy::app::{App, Plugin};

mod default;
mod plugins;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(default::Default)
            .add_plugins(plugins::game::GamePlugins);

        if cfg!(debug_assertions) {
            app.add_plugins(plugins::dev::dev::DevPlugins);
        }
    }
}