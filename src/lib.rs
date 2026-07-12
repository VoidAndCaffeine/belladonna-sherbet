use bevy::app::{App, Plugin};

mod default;
mod plugins;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                default::Default,
                plugins::game::GamePlugins
                ));
    }
}