mod common;
use common::TestApp;
use belladonna_sherbet::plugins::save::SaveData;
use bevy::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_quicksave_creates_save_file() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(5);

    let saves_dir = test_app.temp_dir.path().join("Coffee Constellations/Belladonna Sherbet/Saves");
    let save_path = saves_dir.join("save.toml");

    // File should not exist initially
    assert!(!save_path.exists(), "Save file should not exist before quicksave");

    // Press F5 to quicksave
    test_app.press_key(KeyCode::F5);
    test_app.advance_frames(2);

    // File should exist now
    assert!(save_path.exists(), "Save file should exist after quicksave");
}

#[test]
fn test_quicksave_updates_version() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(5);

    test_app.press_key(KeyCode::F5);
    test_app.advance_frames(2);

    let save_data = test_app.app.world().resource::<bevy_persistent::Persistent<SaveData>>();
    assert_eq!(save_data.version, Some(env!("CARGO_PKG_VERSION").to_string()));
}

#[test]
fn test_quicksave_increments_counter() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(5);

    let save_data = test_app.app.world().resource::<bevy_persistent::Persistent<SaveData>>();
    assert_eq!(save_data.test, None);

    test_app.press_key(KeyCode::F5);
    test_app.advance_frames(2);

    let save_data = test_app.app.world().resource::<bevy_persistent::Persistent<SaveData>>();
    assert_eq!(save_data.test, Some(1));

    test_app.press_key(KeyCode::F5);
    test_app.advance_frames(2);

    let save_data = test_app.app.world().resource::<bevy_persistent::Persistent<SaveData>>();
    assert_eq!(save_data.test, Some(2));
}

#[test]
fn test_quicksave_persists_to_disk() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(5);

    test_app.press_key(KeyCode::F5);
    test_app.advance_frames(2);

    let saves_dir = test_app.temp_dir.path().join("Coffee Constellations/Belladonna Sherbet/Saves");
    let save_path = saves_dir.join("save.toml");

    let content = fs::read_to_string(&save_path).expect("Failed to read save file");
    assert!(content.contains("version"));
    assert!(content.contains("test"));
}

#[test]
fn test_quicksave_works_in_any_state() {
    let mut test_app = TestApp::new();
    test_app.spawn_player();
    test_app.advance_frames(5);

    // Change to Loading state
    test_app.app.world_mut().resource_mut::<bevy::state::state::NextState<belladonna_sherbet::plugins::game::GameState>>()
        .set(belladonna_sherbet::plugins::game::GameState::Loading);
    test_app.advance_frames(2);

    test_app.press_key(KeyCode::F5);
    test_app.advance_frames(2);

    let save_data = test_app.app.world().resource::<bevy_persistent::Persistent<SaveData>>();
    assert_eq!(save_data.test, Some(1));
}