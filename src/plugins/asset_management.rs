use bevy::app::{App, Plugin};
use bevy::prelude::*;
use pipelines_ready::*;
use crate::plugins::game::GameState;
use crate::plugins::location_change::{Location, LocationChange, LocationChangeDest};
use crate::plugins::player::{Player, PlayerSpawnState};

#[derive(Component,Reflect)]
#[reflect(Component)]
struct LightNeedsShadows;
pub struct AssetManagerPlugin;
impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<LightNeedsShadows>()
            .add_observer(add_shadows_to_lights)
            .add_plugins(PipelinesReadyPlugin)
            .init_state::<LoadingState>()
            .insert_resource(LoadingData::new(5))
            .add_systems(Update, update_loading_data)
            .add_systems(Update,unload_current_level.run_if(resource_changed::<LocationChange>))
            .add_systems(
                Update,
                load_new_level
                    .run_if(resource_changed::<LocationChange>)
                    .after(unload_current_level)
            )
            .add_observer(respawn_player.run_if(in_state(PlayerSpawnState::Respawn)))
        ;
    }
}

#[derive(States,Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
enum LoadingState {
    #[default]
    LevelReady,
    LevelLoading,
}

#[derive(Resource,Debug,Default)]
struct LoadingData {
    loading_assets: Vec<UntypedHandle>,
    confirmation_frames_needed:usize,
    confirmation_frames_count:usize,
}

impl LoadingData {
    fn new(confirmation_frames_needed:usize) -> Self {
        Self {
            loading_assets: Vec::new(),
            confirmation_frames_needed,
            confirmation_frames_count: 0
        }
    }
}

#[derive(Component)]
struct LevelComponents;

fn unload_current_level(
    mut commands: Commands,
    mut loading_state: ResMut<NextState<LoadingState>>,
    mut game_state: ResMut<NextState<GameState>>,
    spawn_state: ResMut<State<PlayerSpawnState>>,
    mut spawn_next_state: ResMut<NextState<PlayerSpawnState>>,
    entities: Query<Entity, With<LevelComponents>>,
) {
    if *spawn_state.get() != PlayerSpawnState::NotYetSpawned {
        spawn_next_state.set(PlayerSpawnState::Respawn);
    }
    game_state.set(GameState::Loading);
    loading_state.set(LoadingState::LevelLoading);
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

fn load_new_level(
    mut commands: Commands,
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>,
    location_change_info: ResMut<LocationChange>,
) {
    let level = match location_change_info.destination {
        Location::Test => {
            info!("Loading Test Level");
            asset_server.load(GltfAssetLabel::Scene(1).from_asset("belladonna-sherbet.gltf"))
        }
        Location::Greenhouse => {
            info!("Loading Greenhouse Level");
            asset_server.load(GltfAssetLabel::Scene(2).from_asset("belladonna-sherbet.gltf"))
        }
        Location::CaspianGround => {
            info!("Loading Caspian Ground Floor");
            asset_server.load(GltfAssetLabel::Scene(3).from_asset("belladonna-sherbet.gltf"))
        }
    };
    loading_data.loading_assets.push(level.clone().into());
    commands.spawn((
        WorldAssetRoot(level.clone()),
        LevelComponents,
    ));
}

fn update_loading_data(
    mut loading_data: ResMut<LoadingData>,
    mut loading_state: ResMut<NextState<LoadingState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut spawn_next_state: ResMut<NextState<PlayerSpawnState>>,
    asset_server: Res<AssetServer>,
    pipelines_ready: Res<PipelinesReady>,
){
    if !loading_data.loading_assets.is_empty() || !pipelines_ready.0 {
        loading_data.confirmation_frames_count = 0;
        loading_data.loading_assets.retain(|asset| {
            asset_server
                .get_recursive_dependency_load_state(asset)
                .is_none_or(|state| !state.is_loaded())
        });
    } else {
        loading_data.confirmation_frames_count += 1;
        if loading_data.confirmation_frames_count == loading_data.confirmation_frames_needed {
            loading_state.set(LoadingState::LevelReady);
            spawn_next_state.set(PlayerSpawnState::Spawned);
            game_state.set(GameState::InGame); // ToDo: move this to loading screen once implemented
        }
    }
}

fn add_shadows_to_lights(
    event: On<Add, PointLight>,
    mut commands: Commands,
    mut query: Query<&mut PointLight, With<LightNeedsShadows>>,
) {
    let Ok(mut light) = query.get_mut(event.entity) else {
        warn!("Found a light not needing shadows");
        return
    };
    light.shadow_maps_enabled = true;
    commands.entity(event.entity).remove::<LightNeedsShadows>();
}


fn respawn_player(
    event: On<Add, LocationChangeDest>,
    lc: Res<LocationChange>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    query_t: Query<&Transform, (With<LocationChangeDest>,Without<Player>)>,
    query_lc: Query<&LocationChangeDest>,
) {
    info!("Attempting Respawn Player");
    let lc_info = match query_lc.get(event.entity) {
        Ok(info) => info,
        Err(_) => return,
    };
    if lc.origin != lc_info.origin || lc.destination != lc_info.destination {
        return;
    }
    let transform = match query_t.get(event.entity) {
        Ok(transform) => transform,
        Err(_) => return,
    };
    info!("Respawning player from {:?} in {:?} at position {}",lc_info.origin,lc_info.destination, transform.translation);
    player_transform.translation = transform.translation + Vec3::new(0.0, 0.8, 0.0);
}

mod pipelines_ready {
    use bevy::{
        prelude::*,
        render::{render_resource::*,*}
    };
    pub struct PipelinesReadyPlugin;
    impl Plugin for PipelinesReadyPlugin {
        fn build(&self, app: &mut App) {
            app
                .insert_resource(PipelinesReady::default())
                .sub_app_mut(RenderApp).add_systems(ExtractSchedule,update_pipelines_ready)
            ;
        }
    }
    #[derive(Resource,Debug,Default)]
    pub struct PipelinesReady(pub bool);
    fn update_pipelines_ready(
        mut main_world: ResMut<MainWorld>,
        pipelines: Res<PipelineCache>
    ){
        if let Some(mut pipelines_ready) = main_world.get_resource_mut::<PipelinesReady>() {
            pipelines_ready.0 = pipelines.waiting_pipelines().count() == 0;
        }
    }
}