use belladonna_sherbet::plugins::game::GameState;
use belladonna_sherbet::plugins::player::PlayerSpawnState;
use belladonna_sherbet::plugins::yarn::DialogueState;
use belladonna_sherbet::plugins::asset_management::LoadingState;
use belladonna_sherbet::plugins::location_change::Location;
use bevy::prelude::*;
use bevy_state::app::StatesPlugin;

#[test]
fn test_game_state_default() {
    let state = GameState::default();
    assert_eq!(state, GameState::InGame);
}

#[test]
fn test_game_state_variants() {
    assert_eq!(GameState::InGame as u8, 0);
    assert_eq!(GameState::Loading as u8, 1);
}

#[test]
fn test_player_spawn_state_default() {
    let state = PlayerSpawnState::default();
    assert_eq!(state, PlayerSpawnState::NotYetSpawned);
}

#[test]
fn test_player_spawn_state_variants() {
    assert_eq!(PlayerSpawnState::NotYetSpawned as u8, 0);
    assert_eq!(PlayerSpawnState::Respawn as u8, 1);
    assert_eq!(PlayerSpawnState::Spawned as u8, 2);
}

#[test]
fn test_dialogue_state_default() {
    let state = DialogueState::default();
    assert_eq!(state, DialogueState::Game);
}

#[test]
fn test_dialogue_state_variants() {
    assert_eq!(DialogueState::Game as u8, 0);
    assert_eq!(DialogueState::Dialogue as u8, 1);
}

#[test]
fn test_loading_state_default() {
    let state = LoadingState::default();
    assert_eq!(state, LoadingState::LevelReady);
}

#[test]
fn test_loading_state_variants() {
    assert_eq!(LoadingState::LevelReady as u8, 0);
    assert_eq!(LoadingState::LevelLoading as u8, 1);
}

#[test]
fn test_location_default() {
    let location = Location::default();
    assert_eq!(location, Location::Test);
}

#[test]
fn test_location_variants() {
    assert_eq!(Location::Test as u8, 0);
    assert_eq!(Location::Greenhouse as u8, 1);
    assert_eq!(Location::CaspianGround as u8, 2);
}

#[test]
fn test_state_transitions() {
    let mut app = App::new();
    app.add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .init_state::<PlayerSpawnState>()
        .init_state::<DialogueState>()
        .init_state::<LoadingState>()
        .init_state::<Location>();

    assert_eq!(app.world().resource::<State<GameState>>().get(), &GameState::InGame);
    assert_eq!(app.world().resource::<State<PlayerSpawnState>>().get(), &PlayerSpawnState::NotYetSpawned);
    assert_eq!(app.world().resource::<State<DialogueState>>().get(), &DialogueState::Game);
    assert_eq!(app.world().resource::<State<LoadingState>>().get(), &LoadingState::LevelReady);
    assert_eq!(app.world().resource::<State<Location>>().get(), &Location::Test);

    app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::Loading);
    app.update();
    assert_eq!(app.world().resource::<State<GameState>>().get(), &GameState::Loading);

    app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::InGame);
    app.update();
    assert_eq!(app.world().resource::<State<GameState>>().get(), &GameState::InGame);
}

#[test]
fn test_state_equality() {
    assert_eq!(GameState::InGame, GameState::InGame);
    assert_ne!(GameState::InGame, GameState::Loading);

    assert_eq!(PlayerSpawnState::Spawned, PlayerSpawnState::Spawned);
    assert_ne!(PlayerSpawnState::NotYetSpawned, PlayerSpawnState::Spawned);

    assert_eq!(DialogueState::Game, DialogueState::Game);
    assert_ne!(DialogueState::Game, DialogueState::Dialogue);

    assert_eq!(LoadingState::LevelReady, LoadingState::LevelReady);
    assert_ne!(LoadingState::LevelReady, LoadingState::LevelLoading);

    assert_eq!(Location::Test, Location::Test);
    assert_ne!(Location::Test, Location::Greenhouse);
}

#[test]
fn test_state_clone() {
    let state1 = GameState::Loading;
    let state2 = state1.clone();
    assert_eq!(state1, state2);

    let spawn_state = PlayerSpawnState::Respawn;
    assert_eq!(spawn_state, spawn_state.clone());

    let dialogue_state = DialogueState::Dialogue;
    assert_eq!(dialogue_state, dialogue_state.clone());

    let loading_state = LoadingState::LevelLoading;
    assert_eq!(loading_state, loading_state.clone());

    let location = Location::CaspianGround;
    assert_eq!(location, location.clone());
}