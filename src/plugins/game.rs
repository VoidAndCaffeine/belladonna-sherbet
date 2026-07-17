use bevy::app::*;
use bevy::prelude::*;
use crate::plugins::{camera, player,asset_management,location_change};

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
            .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            .add_plugins(camera::CameraPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(asset_management::AssetManagerPlugin)
            .add_plugins(location_change::LocationChangePlugin)
            .add_systems(Startup,(spawn_test_level))
        ;
    }
}


fn spawn_test_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(2).from_asset("belladonna-sherbet.gltf")),
        ));
    info!("Created Game");
}