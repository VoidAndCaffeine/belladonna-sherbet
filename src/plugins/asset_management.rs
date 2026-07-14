use bevy::app::{App, Plugin};
use bevy::prelude::*;

#[derive(Component,Reflect)]
#[reflect(Component)]
struct LightNeedsShadows;
pub struct AssetManagerPlugin;
impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<LightNeedsShadows>()
            .add_observer(add_shadows_to_lights)
        ;
    }
}

fn add_shadows_to_lights(
    event: On<Add, PointLight>,
    mut commands: Commands,
    mut query: Query<&mut PointLight, With<LightNeedsShadows>>,
) {
    info!("Added light");
    let Ok(mut light) = query.get_mut(event.entity) else {
        info!("Entity was not found");
        return
    };
    info!("Setting shadows on lights");
    light.shadow_maps_enabled = true;
    commands.entity(event.entity).remove::<LightNeedsShadows>();
}