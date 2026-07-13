use avian3d::PhysicsPlugins;
use bevy::prelude::{Plugin, App, DefaultPlugins, FixedUpdate};

pub struct Default;
impl Plugin for Default {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins(PhysicsPlugins::default())
            //Plugins for Tnua added in player.rs
        ;
    }
}