mod player;
use bevy::app::{App, Plugin};

pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app;
    }
}