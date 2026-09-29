mod common;
use common::TestApp;
use bevy::prelude::*;
use serial_test::serial;

#[test]
#[serial]
fn test_camera_spawns_with_player_camera_component() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(2);

    let camera_transform = test_app.get_camera_transform();
    assert!(camera_transform.is_some(), "Camera should be spawned");
}

#[test]
#[serial]
fn test_camera_follows_player_position() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(2);

    let player_transform = test_app.get_player_transform().expect("Player should exist");
    let camera_transform = test_app.get_camera_transform().expect("Camera should exist");

    // Camera should be at player position + offset
    let expected_offset = belladonna_sherbet::plugins::camera::CAMERA_DISTANCE * belladonna_sherbet::plugins::camera::CAMERA_VECTOR;
    let expected_pos = player_transform.translation + expected_offset;

    assert!((camera_transform.translation - expected_pos).length() < 0.1,
        "Camera should follow player with offset. Expected: {:?}, Got: {:?}", expected_pos, camera_transform.translation);
}

#[test]
#[serial]
fn test_camera_looks_at_player() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(2);

    let player_transform = test_app.get_player_transform().expect("Player should exist");
    let camera_transform = test_app.get_camera_transform().expect("Camera should exist");

    // Camera forward vector should point toward player
    let to_player = (player_transform.translation - camera_transform.translation).normalize();
    let camera_forward = camera_transform.forward().normalize();

    // Dot product should be close to 1 (same direction)
    let dot = to_player.dot(camera_forward);
    assert!(dot > 0.99, "Camera should look at player. Dot product: {}", dot);
}

#[test]
#[serial]
fn test_camera_maintains_distance() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(2);

    let player_transform = test_app.get_player_transform().expect("Player should exist");
    let camera_transform = test_app.get_camera_transform().expect("Camera should exist");

    let distance = (player_transform.translation - camera_transform.translation).length();
    let expected_distance = belladonna_sherbet::plugins::camera::CAMERA_DISTANCE;

    assert!((distance - expected_distance).abs() < 0.1,
        "Camera should maintain distance. Expected: {}, Got: {}", expected_distance, distance);
}