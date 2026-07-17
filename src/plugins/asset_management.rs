use avian3d::prelude::{LockedAxes, RigidBody};
use bevy::app::{App, Plugin};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_tnua::{TnuaConfig, TnuaController};
use bevy_tnua::builtins::TnuaBuiltinWalkConfig;
use pipelines_ready::*;
use crate::plugins::camera::PlayerCamera;
use crate::plugins::location_change::{Location, LocationChange, LocationChangeDest, LocationChangeInfo};
use crate::plugins::player::Player;

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
            .insert_resource(LoadingState::default())
            .insert_resource(LoadingData::new(5))
            .add_systems(Update, update_loading_data)
            .add_systems(Update,unload_current_level.run_if(resource_changed::<LocationChange>))
            .add_systems(
                Update,
                load_new_level
                    .run_if(resource_changed::<LocationChange>)
                    .after(unload_current_level)
            )
            .add_observer(spawn_player)
        ;
    }
}

#[derive(Resource,Default)]
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
    mut loading_state: ResMut<LoadingState>,
    entities: Query<Entity, With<LevelComponents>>,
) {
    *loading_state = LoadingState::LevelLoading;
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
    };
    loading_data.loading_assets.push(level.clone().into());
    commands.spawn((
        WorldAssetRoot(level.clone()),
        LevelComponents,
    ));
}

fn update_loading_data(
    mut loading_data: ResMut<LoadingData>,
    mut loading_state: ResMut<LoadingState>,
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
            *loading_state = LoadingState::LevelReady;
        }
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

fn spawn_player(
    event: On<Add, LocationChangeDest>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut control_scheme_configs: ResMut<Assets<crate::plugins::player::ControlSchemeConfig>>,
    lc: Res<LocationChange>,
    query_t: Query<&Transform, With<LocationChangeDest>>,
    query_lc: Query<&LocationChangeDest>
) {
    let lc_info = match query_lc.get(event.entity) {
        Ok(info) => info,
        Err(_) => return,
    };
    if lc.origin != lc_info.origin || lc.destination != lc_info.destination {
        info!("Could Not find a matching spawn point");
        info!("Found {:?}, {:?}; expected: {:?}, {:?}", lc.origin, lc.destination, lc_info.origin, lc_info.destination);
        return;
    }
    let transform = match query_t.get(event.entity) {
        Ok(transform) => transform,
        Err(_) => return,
    };
    info!("Spawning player from {:?} in {:?} at position {}",lc_info.origin,lc_info.destination, transform.translation);
    let child
        = asset_server.load(GltfAssetLabel::Scene(0).from_asset("belladonna-sherbet.gltf"));
    commands.spawn((
        WorldAssetRoot(child),
        LevelComponents,
        Transform::from_translation(transform.translation),
        Player,
        RigidBody::Dynamic,
        TnuaController::<crate::plugins::player::ControlScheme>::default(),
        TnuaConfig::<crate::plugins::player::ControlScheme>(control_scheme_configs.add( crate::plugins::player::ControlSchemeConfig {
            basis: TnuaBuiltinWalkConfig {
                float_height:0.01,
                ..Default::default()
            }
        })),
        //  TnuaAvian3dSensorShape(Collider::cylinder(0.49,0.0)),
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
    ));
    commands.spawn((
        PlayerCamera,
        LevelComponents,
        Camera {
            order: 100,
            ..default()
        },
        AmbientLight{
            brightness:0.00,
            ..default()
        },
        Bloom::NATURAL,
    ));
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