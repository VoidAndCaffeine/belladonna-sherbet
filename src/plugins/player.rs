use bevy::app::App;
use bevy::prelude::{ReflectComponent, Component, Reflect, Plugin};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
        ;
    }
}