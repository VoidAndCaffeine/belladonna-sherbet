use bevy::app::{App, Plugin};

mod player;

pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(player::PlayerPlugin)
        ;
    }
}