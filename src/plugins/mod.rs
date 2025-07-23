use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::gltf::GltfAssetLabel;
use bevy::prelude::{Commands, NextState, Res, ResMut};
use bevy::scene::SceneRoot;
use crate::plugins::asset_managment::LoadingState;

pub mod player;
pub mod asset_managment;
mod dev;

pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(player::PlayerPlugin)
            .add_plugins(asset_managment::AssetManagerPlugin)
            .add_systems(Startup, spawn_test_level)
        ;

        // this is only needed to send components to blender,
        // thus it is not needed in release builds
        #[cfg(feature = "dev")]
        app.add_plugins(dev::DevPlugins);
    }
}

fn spawn_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_state: ResMut<LoadingState>,
){
    *loading_state = LoadingState::Loading;
    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(1).from_asset("belladonna-sherbet.gltf"))
    ));
}
