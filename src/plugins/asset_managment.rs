use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use std::collections::VecDeque;
use avian3d::parry::either::IntoEither;
use pipelines_ready::*;
use crate::prelude::{game::GameState, asset_managment::LoadingState::Loading};

#[derive(Component)]
struct LevelComponent;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Component,Reflect)]
#[reflect(Component)]
struct LightNeedsShadows;

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
            .register_type::<LightNeedsShadows>()
            .insert_resource(LoadingData::new(5))
            .init_state::<LoadingState>()
            .add_observer(add_shadows_to_lights)
            .add_plugins(PipelinesReadyPlugin)
            .add_systems(Update, update_loading_data)
        ;
    }
}

fn unload_assets<T: bevy::prelude::Component>(
    mut commands: Commands,
    entities: Query<Entity, With<T>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_loading_data(
    mut loading_data: ResMut<LoadingData>,
    mut loading_state: ResMut<NextState<LoadingState>>,
    mut game_state: ResMut<NextState<GameState>>,
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
            loading_state.set(LoadingState::Ready);
            game_state.set(GameState::InGame);
            info!("Loaded all assets");
        }
    }
}

fn add_shadows_to_lights(
    trigger: Trigger<OnAdd, PointLight>,
    mut query: Query<&mut PointLight, With<LightNeedsShadows>>,
){
    let Ok(mut light) = query.get_mut(trigger.target()) else {return;};
    info!("Setting shadows on light");
    light.shadows_enabled = true;
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