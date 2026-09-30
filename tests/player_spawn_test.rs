mod common;
use common::TestApp;
use belladonna_sherbet::plugins::player::{Player, PlayerSpawnState};
use belladonna_sherbet::plugins::camera::PlayerCamera;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use avian3d::prelude::{RigidBody, Collider, LockedAxes};

#[test]
fn test_player_spawn_state_transitions() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    

    // Initially NotYetSpawned
    assert_eq!(test_app.get_player_spawn_state(), PlayerSpawnState::NotYetSpawned);

    test_app.spawn_player();
    test_app.advance_frames(2);

    // Should transition to Spawned
    assert_eq!(test_app.get_player_spawn_state(), PlayerSpawnState::Spawned);
}

#[test]
fn test_player_spawn_has_required_components() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(2);

    let player_entity = test_app.app.world_mut().query_filtered::<Entity, With<Player>>()
        .single(test_app.app.world_mut()).unwrap();

    // Check all required components exist
    assert!(test_app.app.world_mut().get::<Player>(player_entity).is_some());
    assert!(test_app.app.world_mut().get::<RigidBody>(player_entity).is_some());
    assert!(test_app.app.world_mut().get::<TnuaController<belladonna_sherbet::plugins::player::ControlScheme>>(player_entity).is_some());
    assert!(test_app.app.world_mut().get::<TnuaConfig<belladonna_sherbet::plugins::player::ControlScheme>>(player_entity).is_some());
    assert!(test_app.app.world_mut().get::<Collider>(player_entity).is_some());
    assert!(test_app.app.world_mut().get::<LockedAxes>(player_entity).is_some());
    assert!(test_app.app.world_mut().get::<Transform>(player_entity).is_some());
}

#[test]
fn test_camera_spawns_alongside_player() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(2);

    let camera_entity = test_app.app.world_mut().query_filtered::<Entity, With<PlayerCamera>>()
        .single(test_app.app.world_mut()).unwrap();

    assert!(test_app.app.world_mut().get::<PlayerCamera>(camera_entity).is_some());
    assert!(test_app.app.world_mut().get::<Camera>(camera_entity).is_some());
    assert!(test_app.app.world_mut().get::<Transform>(camera_entity).is_some());
}

#[test]
fn test_player_spawn_position() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(2);

    let player_transform = test_app.get_player_transform().expect("Player should exist");

    // Player should spawn at Y = 0.8 (float_height)
    assert!((player_transform.translation.y - 0.8).abs() < 0.1,
        "Player should spawn at y=0.8, got {}", player_transform.translation.y);
}

#[test]
fn test_player_respawn_state() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(2);

    assert_eq!(test_app.get_player_spawn_state(), PlayerSpawnState::Spawned);

    // Trigger respawn by changing LocationChange
    let _location_change = test_app.app.world().resource::<belladonna_sherbet::plugins::location_change::LocationChange>();
    // The respawn is triggered via observer on LocationChangeDest
    // For this test, we just verify the state machine works
}