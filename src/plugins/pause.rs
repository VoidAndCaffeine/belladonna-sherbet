use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[derive(Resource,Default)]
pub enum PauseState{
    #[default]
    Paused,
    Unpaused,
}

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
        ;
    }
}