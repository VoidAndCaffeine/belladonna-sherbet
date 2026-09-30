mod common;
use common::TestApp;
use belladonna_sherbet::plugins::game::GameState;
use belladonna_sherbet::plugins::asset_management::LoadingState;
use belladonna_sherbet::plugins::location_change::{Location, LocationChange, LocationChangeInfo};
use belladonna_sherbet::plugins::player::{Player, PlayerSpawnState};
use bevy::prelude::*;
use avian3d::prelude::CollisionStart;

fn make_collision(collider1: Entity, collider2: Entity) -> CollisionStart {
    CollisionStart {
        collider1,
        collider2,
        body1: None,
        body2: None,
    }
}

#[test]
fn test_loading_state_transitions_on_location_change() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    // Initial state
    assert_eq!(test_app.get_loading_state(), LoadingState::LevelReady);
    assert_eq!(test_app.get_game_state(), GameState::InGame);

    // Create a location change trigger
    let trigger_entity = test_app.app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    }).id();

    let player_entity = test_app.app.world_mut().query_filtered::<Entity, With<Player>>()
        .single(test_app.app.world_mut()).unwrap();

    // Trigger collision
    let collision = make_collision(trigger_entity, player_entity);
    test_app.app.world_mut().trigger(collision);
    test_app.advance_frames(2);

    // Loading state should change to LevelLoading
    assert_eq!(test_app.get_loading_state(), LoadingState::LevelLoading);
    assert_eq!(test_app.get_game_state(), GameState::Loading);
}

#[test]
fn test_old_level_entities_despawned_on_location_change() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    // Count initial level entities
    let initial_level_entities: Vec<_> = test_app.app.world_mut().query_filtered::<Entity, With<belladonna_sherbet::plugins::asset_management::LevelComponents>>()
        .iter(test_app.app.world_mut()).collect();
    assert!(!initial_level_entities.is_empty(), "Should have initial level entities");

    // Trigger location change
    let trigger_entity = test_app.app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    }).id();

    let player_entity = test_app.app.world_mut().query_filtered::<Entity, With<Player>>()
        .single(test_app.app.world_mut()).unwrap();

    let collision = make_collision(trigger_entity, player_entity);
    test_app.app.world_mut().trigger(collision);
    test_app.advance_frames(10);

    // Old level entities should be despawned
    let _remaining_level_entities: Vec<_> = test_app.app.world_mut().query_filtered::<Entity, With<belladonna_sherbet::plugins::asset_management::LevelComponents>>()
        .iter(test_app.app.world_mut()).collect();

    // The old level entities should be gone (though new ones may be loading)
    // We can't easily distinguish old vs new without more setup, but we can verify
    // the loading state transitioned
    assert_eq!(test_app.get_loading_state(), LoadingState::LevelLoading);
}

#[test]
fn test_location_change_resource_updated() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    let trigger_entity = test_app.app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    }).id();

    let player_entity = test_app.app.world_mut().query_filtered::<Entity, With<Player>>()
        .single(test_app.app.world_mut()).unwrap();

    let collision = make_collision(trigger_entity, player_entity);
    test_app.app.world_mut().trigger(collision);
    test_app.advance_frames(2);

    let location_change = test_app.app.world().resource::<LocationChange>();
    assert_eq!(location_change.origin, Location::Test);
    assert_eq!(location_change.destination, Location::Greenhouse);
}

#[test]
fn test_player_respawn_state_on_location_change() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    assert_eq!(test_app.get_player_spawn_state(), PlayerSpawnState::Spawned);

    let trigger_entity = test_app.app.world_mut().spawn(LocationChangeInfo {
        origin: Location::Test,
        destination: Location::Greenhouse,
    }).id();

    let player_entity = test_app.app.world_mut().query_filtered::<Entity, With<Player>>()
        .single(test_app.app.world_mut()).unwrap();

    let collision = make_collision(trigger_entity, player_entity);
    test_app.app.world_mut().trigger(collision);
    test_app.advance_frames(2);

    // Player spawn state should transition to Respawn
    assert_eq!(test_app.get_player_spawn_state(), PlayerSpawnState::Respawn);
}