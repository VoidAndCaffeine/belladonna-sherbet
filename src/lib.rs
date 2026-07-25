use bevy::app::{App, Plugin};
#[cfg(feature = "dev-tools")]
use crate::plugins::dev::inspector::InspectorDevPlugin;

mod default;
mod plugins;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(default::Default)
            .add_plugins(plugins::game::GamePlugins)
            .add_plugins(bevy_skein::SkeinPlugin::default())

        ;

        #[cfg(feature = "dev-tools")]
        app
            .add_plugins(InspectorDevPlugin)
        ;
    }
}
