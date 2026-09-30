use serial_test::serial;
use bevy::prelude::*;
use bevy_state::app::StatesPlugin;
use bevy_persistent::Persistent;
use belladonna_sherbet::plugins::save::{SaveData, SavePlugin};
use belladonna_sherbet::plugins::game::{DataPath, GameState};
use tempfile::TempDir;

fn setup_test_app() -> (App, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let save_dir = temp_dir.path().join("Saves");
    std::fs::create_dir_all(&save_dir).unwrap();

    let mut app = App::new();
    app.add_plugins(StatesPlugin)
        .init_resource::<ButtonInput<KeyCode>>()
        .insert_resource(DataPath { path: temp_dir.path().to_path_buf() })
        .init_state::<GameState>()
        .add_plugins(SavePlugin);
    app.update();
    (app, temp_dir)
}

#[test]
#[serial]
fn test_save_plugin_setup_creates_persistent_resource() {
    let (app, _temp_dir) = setup_test_app();
    let save_data = app.world().resource::<Persistent<SaveData>>();
    assert!(save_data.version.is_none());
    assert!(save_data.test.is_none());
}

#[test]
#[serial]
fn test_save_plugin_default_save_data() {
    let (app, _temp_dir) = setup_test_app();
    let save_data = app.world().resource::<Persistent<SaveData>>();
    assert_eq!(save_data.version, None);
    assert_eq!(save_data.test, None);
}

#[test]
#[serial]
fn test_quicksave_increments_test_counter() {
    let (mut app, _temp_dir) = setup_test_app();
    {
        let save_data = app.world().resource::<Persistent<SaveData>>();
        assert_eq!(save_data.test, None);
    }
    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F5);
    app.update();
    {
        let save_data = app.world().resource::<Persistent<SaveData>>();
        assert_eq!(save_data.test, Some(1));
    }
}

#[test]
#[serial]
fn test_quicksave_sets_version() {
    let (mut app, _temp_dir) = setup_test_app();
    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F5);
    app.update();
    let save_data = app.world().resource::<Persistent<SaveData>>();
    assert_eq!(save_data.version, Some(env!("CARGO_PKG_VERSION").to_string()));
}

#[test]
#[serial]
fn test_quicksave_persists_to_file() {
    let (mut app, temp_dir) = setup_test_app();
    let save_path = temp_dir.path().join("Saves").join("save.toml");
    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F5);
    app.update();
    assert!(save_path.exists());
    let contents = std::fs::read_to_string(&save_path).unwrap();
    assert!(contents.contains("test = 1"));
    assert!(contents.contains(&format!("version = \"{}\"", env!("CARGO_PKG_VERSION"))));
}

#[test]
#[serial]
fn test_multiple_quicksaves_increment_counter() {
    let (mut app, _temp_dir) = setup_test_app();
    for i in 1..=3 {
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F5);
        app.update();
        let save_data = app.world().resource::<Persistent<SaveData>>();
        assert_eq!(save_data.test, Some(i));
    }
}

#[test]
#[serial]
fn test_quicksave_only_on_f5_press() {
    let (mut app, _temp_dir) = setup_test_app();
    app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F1);
    app.update();
    let save_data = app.world().resource::<Persistent<SaveData>>();
    assert_eq!(save_data.test, None);
}

#[test]
#[serial]
fn test_save_data_toml_roundtrip() {
    let mut original = SaveData::default();
    original.version = Some("1.0.0".to_string());
    original.test = Some(5);

    let toml_str = toml::to_string_pretty(&original).unwrap();
    let parsed: SaveData = toml::from_str(&toml_str).unwrap();

    assert_eq!(original.version, parsed.version);
    assert_eq!(original.test, parsed.test);
}

#[test]
#[serial]
fn test_save_data_skip_serializing_none() {
    let save_data = SaveData::default();

    let toml_str = toml::to_string(&save_data).unwrap();
    assert!(!toml_str.contains("version"));
    assert!(!toml_str.contains("test"));
}