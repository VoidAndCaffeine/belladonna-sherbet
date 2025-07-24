use bevy::app::{App, Plugin};
use bevy_skein::SkeinPlugin;

pub struct SkeinDevPlugin;
impl Plugin for SkeinDevPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SkeinPlugin::default())
        ;
    }
}