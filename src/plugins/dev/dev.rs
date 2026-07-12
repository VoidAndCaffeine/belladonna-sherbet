use bevy::app::{App, Plugin};
use crate::plugins::dev::inspector::InspectorPlugin;

pub struct DevPlugins;
impl Plugin for DevPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InspectorPlugin)
        ;
    }
}