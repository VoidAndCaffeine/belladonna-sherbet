use bevy::app::App;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Component, Plugin, Reflect, ReflectComponent, Res};

// create component to store asset path
// insert into bevy

// add empty with asset component
// in blender instance library collection

// https://docs.rs/bevy/latest/bevy/gltf/index.html
// load gltf and insert
// spawn gltf function
// for nodes with asset path component use location to get position to load the correct asset
#[derive(Component,Reflect)]
#[reflect(Component)]
pub struct GltfRef{
    file: String,
}

pub struct SceneBuilderPlugin;
impl Plugin for SceneBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<GltfRef>()
        ;
    }
}

fn test_load_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("blender/Test.gltf");

}