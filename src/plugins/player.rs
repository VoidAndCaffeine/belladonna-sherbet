use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::prelude::*;
use crate::prelude::asset_managment::LoadingData;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerSpawn;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Player>()
            .register_type::<PlayerSpawn>()
            .add_observer(spawn_player)
        ;
    }
}
fn spawn_player(
    trigger: Trigger<OnAdd, PlayerSpawn>,
    query: Query<&Transform>,
    mut commands: Commands,
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>,
) {
    let transform = query.get(trigger.target()).unwrap();
    info!("spawning player");
    let child
        = asset_server.load(GltfAssetLabel::Scene(0).from_asset("belladonna-sherbet.gltf"));
    loading_data.loading_assets.push(child.clone().into());
    commands.spawn((
        SceneRoot(child),
        *transform,
        Player,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
        ));
}

