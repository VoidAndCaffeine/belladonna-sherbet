use bevy::app::*;
use bevy::prelude::*;
use crate::plugins::{camera, player,asset_management,location_change,yarn};

//TODO: move loading to in game to loading screen
// currently set in asset_management, update_loading_data
#[derive(States,Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState{
    #[default]
    InGame,
    Loading,
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
            .add_plugins(yarn::YarnPlugin)
        ;
    }
}