use std::collections::VecDeque;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;

#[derive(Component)]
struct LevelComponent;

/// Component to be added to blender empties
#[derive(Component,Reflect)]
#[reflect(Component)]
pub struct GltfRef{
    collection: String,
}

#[derive(Resource, Default)]
enum LoadingState{
    #[default]
    Ready,
    Loading,
}

#[derive(Resource, Debug, Default)]
struct LoadingData {
    loading_assets: Vec<UntypedHandle>,
    // for when some actions leave the loading idle for a few frames
    confirmation_frames_target: usize,
    confirmation_frames_count: usize,
}

impl LoadingData {
    fn new(confirmation_frames_target: usize) -> Self {
        Self{
            loading_assets: Vec::new(),
            confirmation_frames_target,
            confirmation_frames_count: 0,
        }
    }
}

#[derive(Resource)]
struct LevelData {
    unload_level_id: SystemId,
    level_1_id: SystemId,
    level_2_id: SystemId,
}

/// Plugin for asset manager systems
pub struct AssetManagerPlugin;
impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<GltfRef>()
            .add_observer(add_gltf_children)
            .add_systems(Startup, spawn_test)
        ;
    }
}

/// system that replaces gltf ref links
fn add_gltf_children(
    trigger: Trigger<OnAdd, GltfRef>,
    query: Query<(&GltfRef, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let gltf_ref = query.get(trigger.target()).unwrap();
    let file = format!("blender/{}.glb", gltf_ref.0.collection.as_str());

    let child = commands.spawn((
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(file))),
        *gltf_ref.1
        )).id();

    commands.entity(trigger.target()).remove::<GltfRef>().add_child(child);
}







pub fn spawn_test(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("blender/test.gltf" ),
        )));
}
