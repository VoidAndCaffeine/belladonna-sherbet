use bevy::app::{App, Plugin};

mod skein;
mod inspector;

pub struct DevPlugins;
impl Plugin for DevPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            skein::SkeinDevPlugin,
            inspector::InspectorDevPlugin,
            ));
    }
}