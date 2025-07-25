use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_inspector_egui::{quick::WorldInspectorPlugin, bevy_egui::EguiPlugin};

pub struct InspectorDevPlugin;
impl Plugin for InspectorDevPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new())
        ;
    }
}