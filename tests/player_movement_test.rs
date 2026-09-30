mod common;
use common::TestApp;
use bevy::prelude::*;

#[test]
fn test_player_moves_forward_with_w_key() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    let initial_pos = test_app.get_player_transform().expect("Player should exist").translation;

    test_app.press_key(KeyCode::KeyW);
    test_app.advance_frames(30); // ~0.5 seconds at 60fps

    let final_pos = test_app.get_player_transform().expect("Player should exist").translation;

    // Player should move forward (negative Z in Bevy's coordinate system)
    assert!(final_pos.z < initial_pos.z - 0.1,
        "Player should move forward. Initial Z: {}, Final Z: {}", initial_pos.z, final_pos.z);
}

#[test]
fn test_player_moves_backward_with_s_key() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    let initial_pos = test_app.get_player_transform().expect("Player should exist").translation;

    test_app.press_key(KeyCode::KeyS);
    test_app.advance_frames(30);

    let final_pos = test_app.get_player_transform().expect("Player should exist").translation;

    assert!(final_pos.z > initial_pos.z + 0.1,
        "Player should move backward. Initial Z: {}, Final Z: {}", initial_pos.z, final_pos.z);
}

#[test]
fn test_player_moves_left_with_a_key() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    let initial_pos = test_app.get_player_transform().expect("Player should exist").translation;

    test_app.press_key(KeyCode::KeyA);
    test_app.advance_frames(30);

    let final_pos = test_app.get_player_transform().expect("Player should exist").translation;

    assert!(final_pos.x < initial_pos.x - 0.1,
        "Player should move left. Initial X: {}, Final X: {}", initial_pos.x, final_pos.x);
}

#[test]
fn test_player_moves_right_with_d_key() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    let initial_pos = test_app.get_player_transform().expect("Player should exist").translation;

    test_app.press_key(KeyCode::KeyD);
    test_app.advance_frames(30);

    let final_pos = test_app.get_player_transform().expect("Player should exist").translation;

    assert!(final_pos.x > initial_pos.x + 0.1,
        "Player should move right. Initial X: {}, Final X: {}", initial_pos.x, final_pos.x);
}

#[test]
fn test_player_stops_when_keys_released() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    test_app.press_key(KeyCode::KeyW);
    test_app.advance_frames(20);

    let moving_pos = test_app.get_player_transform().expect("Player should exist").translation;

    test_app.release_key(KeyCode::KeyW);
    test_app.advance_frames(20);

    let stopped_pos = test_app.get_player_transform().expect("Player should exist").translation;

    // Position should not change significantly after key release
    let drift = (stopped_pos - moving_pos).length();
    assert!(drift < 0.1,
        "Player should stop when key released. Drift: {}", drift);
}

#[test]
fn test_movement_only_in_game_state() {
    let mut test_app = TestApp::shared();
    test_app.reset();
    
    test_app.spawn_player();
    test_app.advance_frames(5);

    // Change to Loading state
    test_app.app.world_mut().resource_mut::<bevy::state::state::NextState<belladonna_sherbet::plugins::game::GameState>>()
        .set(belladonna_sherbet::plugins::game::GameState::Loading);
    test_app.advance_frames(2);

    let initial_pos = test_app.get_player_transform().expect("Player should exist").translation;

    test_app.press_key(KeyCode::KeyW);
    test_app.advance_frames(30);

    let final_pos = test_app.get_player_transform().expect("Player should exist").translation;

    // Player should not move in Loading state
    let movement = (final_pos - initial_pos).length();
    assert!(movement < 0.1,
        "Player should not move in Loading state. Movement: {}", movement);
}