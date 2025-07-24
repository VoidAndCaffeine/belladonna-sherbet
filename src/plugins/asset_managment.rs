use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use std::collections::VecDeque;
use pipelines_ready::*;
use crate::plugins::asset_managment::LoadingState::Loading;

#[derive(Component)]
pub struct LevelComponent;

#[derive(Resource, Default)]
pub enum LoadingState {
    #[default]
    Loading,
    Ready,
}

#[derive(Resource, Debug, Default)]
pub struct LoadingData {
    pub(crate) loading_assets: Vec<UntypedHandle>,
    // for when some actions leave the loading idle for a few frames
    confirmation_frames_target: usize,
    confirmation_frames_count: usize,
}

impl LoadingData {
    fn new(confirmation_frames_target: usize) -> Self {
        Self {
            loading_assets: Vec::new(),
            confirmation_frames_target,
            confirmation_frames_count: 0,
        }
    }
}

/// Plugin for asset manager systems
pub struct AssetManagerPlugin;
impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PipelinesReadyPlugin)
            .insert_resource(LoadingState::default())
            .insert_resource(LoadingData::new(5))
            .add_systems(Update, update_loading_data)
        ;
    }
}
// not actually necessary
/*

/// Component to be added to blender empties
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GltfRef {
    collection: String,
}

/// system that replaces gltf ref links
fn add_gltf_children(
    trigger: Trigger<OnAdd, GltfRef>,
    query: Query<(&GltfRef, &Transform)>,
    mut commands: Commands,
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>,
) {
    let gltf_ref = query.get(trigger.target()).unwrap();
    let file = format!("blender/{}.glb", gltf_ref.0.collection);

    info!("loading {}.glb",gltf_ref.0.collection);
    let child = asset_server.load(GltfAssetLabel::Scene(0).from_asset(file));
    loading_data.loading_assets.push(child.clone().into());
    let child = commands
        .spawn((
            SceneRoot(child.clone()),
            *gltf_ref.1,
        ))
        .id();

    commands
        .entity(trigger.target())
        .remove::<GltfRef>()
        .add_child(child);
}
fn unload_assets<T: bevy::prelude::Component>(
    mut commands: Commands,
    entities: Query<Entity, With<T>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
*/


fn update_loading_data(
    mut loading_data: ResMut<LoadingData>,
    mut loading_state: ResMut<LoadingState>,
    asset_server: Res<AssetServer>,
    pipelines_ready: Res<PipelinesReady>,
){
    if !loading_data.loading_assets.is_empty() || !pipelines_ready.0 {
        loading_data.confirmation_frames_count = 0;
        // remove all loaded
        loading_data.loading_assets.retain( |asset|{
            asset_server
                .get_recursive_dependency_load_state(asset)
                .is_none_or(|state| !state.is_loaded() )
        });
    } else {
        loading_data.confirmation_frames_count += 1;
        if loading_data.confirmation_frames_count
            == loading_data.confirmation_frames_target {
            *loading_state = LoadingState::Ready;
            info!("Loaded all assets");
        }
    }
}

mod pipelines_ready {
    use bevy::{
        prelude::*,
        render::*,
    };
    use bevy::render::render_resource::PipelineCache;

    #[derive(Resource, Debug, Default)]
    pub struct PipelinesReady(pub bool);
    fn update_pipelines_ready(
        mut main_world: ResMut<MainWorld>,
        pipelines: Res<PipelineCache>
    ){
        if let Some(mut piplines_ready)
            = main_world.get_resource_mut::<PipelinesReady>() {
            piplines_ready.0 = pipelines.waiting_pipelines().count() == 0;
        }
    }

    pub struct PipelinesReadyPlugin;
    impl Plugin for PipelinesReadyPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(PipelinesReady::default());
            app.sub_app_mut(RenderApp)
                .add_systems(ExtractSchedule, update_pipelines_ready);
        }
    }
}