use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;
use crate::plugins::{camera, pause};
use crate::prelude::*;

//TODO: move loading to in game to loading screen
// currently set in asset_management, update_loading_data
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState{
    InGame,
    #[default]
    Loading,
}
pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins(player::PlayerPlugin)
            .add_plugins(asset_managment::AssetManagerPlugin)
            .add_plugins(input::InputManagerPlugin)
            .add_plugins(pause::PausePlugin)
            .add_plugins(camera::CameraPlugin)
            .add_systems(Startup, spawn_test_level)
        ;

        // this is only needed to send components to blender,
        // thus it is not needed in release builds
        #[cfg(feature = "dev")]
        app.add_plugins(dev::DevPlugins);
    }
}

//ToDo: Load main menu
fn spawn_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_state: ResMut<NextState<asset_managment::LoadingState>>,
){
    loading_state.set( asset_managment::LoadingState::Loading);
    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(1).from_asset("belladonna-sherbet.gltf"))
    ));
}
