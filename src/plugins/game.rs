use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;
use crate::plugins::{camera, pause, main_menu};
use crate::plugins::asset_managment::{unload_assets, LoadingType};
use crate::prelude::*;
use crate::prelude::asset_managment::{LevelComponent, LoadingState};

//TODO: move loading to in game to loading screen
// currently set in asset_management, update_loading_data
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState{
    MainMenu,
    InGame,
    #[default]
    Loading
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
            .add_plugins(main_menu::MainMenuPlugin)
            .add_systems(OnEnter(LoadingType::LoadingGame), spawn_test_level)
            .add_systems(OnExit(GameState::InGame), unload_assets::<LevelComponent>)
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
    mut loading_state: ResMut<NextState<LoadingState>>,
) {
    loading_state.set(LoadingState::Loading);
    commands.spawn((
        LevelComponent,
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(1).from_asset("belladonna-sherbet.gltf")),
        ),
    ));
    
}
