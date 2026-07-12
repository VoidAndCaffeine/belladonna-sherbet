use bevy::app::{App, Plugin};

mod default;
mod plugins;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(default::Default)
            .add_plugins(plugins::game::GamePlugins);

        #[cfg(feature = "dev-tools")]
        app
            .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
            .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
            .add_plugins(bevy_skein::SkeinPlugin::default());
        ;
    }
}