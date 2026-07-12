use bevy::prelude::*;
pub struct Default;
impl Plugin for Default {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
        ;
    }
}