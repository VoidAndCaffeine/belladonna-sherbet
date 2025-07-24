use bevy::prelude::{App, Plugin};
use bevy_skein::SkeinPlugin;

pub struct DevPlugins;
impl Plugin for DevPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(SkeinPlugin::default());
    }
}