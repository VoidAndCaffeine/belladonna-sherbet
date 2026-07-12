use bevy::app::App;
use bevy::prelude::Plugin;

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_inspector, egui, DefaultInspectorConfigPlugin};
use bevy_inspector_egui::bevy_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin::default())
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_systems(EguiPrimaryContextPass, custom_world_inspector_ui)
        ;
    }
}


fn custom_world_inspector_ui(world: &mut World) {
    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("World Inspector")
        .default_size((400., 300.))
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_world(world, ui);
                ui.allocate_space(ui.available_size());
            });
        });
}
