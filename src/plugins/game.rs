use bevy::app::*;
use bevy::prelude::*;
use crate::plugins::camera;

//TODO: move loading to in game to loading screen
// currently set in asset_management, update_loading_data
#[derive(States,Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState{
    #[default]
    InGame,
}
pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App){
        app
            .init_state::<GameState>()
            .add_plugins(camera::CameraPlugin)
            .add_systems(Startup,(spawn_test_level))
        ;
    }
}


fn spawn_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(1).from_asset("belladonna-sherbet.gltf")),
        )));
    info!("Created Game");
}