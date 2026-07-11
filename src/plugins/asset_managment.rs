use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use std::collections::VecDeque;
use avian3d::parry::either::IntoEither;
use pipelines_ready::*;
use crate::prelude::{game::GameState, asset_managment::LoadingState::Loading};

#[derive(Component)]
pub struct LevelComponent;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadingType{
    LoadingGame,
    #[default]
    LoadingMainMenu,
}

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
            .init_state::<LoadingType>()
            .add_observer(add_shadows_to_lights)
            .add_plugins(PipelinesReadyPlugin)
            .add_systems(Startup, load_loading_screen)
            .add_systems(Update, display_loading_screen)
            .add_systems(Update, update_loading_data)
        ;
    }
}

pub fn unload_assets<T: bevy::prelude::Component>(
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
    loading_type: ResMut<State<LoadingType>>,
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
            match loading_type.get() {
                LoadingType::LoadingGame => {game_state.set(GameState::InGame)}
                LoadingType::LoadingMainMenu => {game_state.set(GameState::MainMenu)}
            }
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

#[derive(Component,Default)]
struct LoadingScreenComponent;

#[derive(Component)]
#[require(Camera2d)]
#[require(LoadingScreenComponent)]
struct LoadingScreenCamera;
fn load_loading_screen(mut commands: Commands){
    let text_style = TextFont{
        font_size: 67.0,
        ..default()
    };

    commands.spawn((
        LoadingScreenCamera,
        Camera{
            order: 99,
            ..default()
        },
    ));
    commands.spawn((
        LoadingScreenComponent,
        Node {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::BLACK),
        children![(Text::new("Loading"), text_style)],
    ));
}

fn display_loading_screen(
    mut loading_screen: Single<&mut Visibility, (With<LoadingScreenComponent>, With<Node>)>,
    loading_state: Res<State<LoadingState>>,
){
    let visibility = match loading_state.get() {
        LoadingState::Loading => Visibility::Visible,
        LoadingState::Ready => Visibility::Hidden,
    };
    **loading_screen = visibility;
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
