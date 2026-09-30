use std::path::PathBuf;

use avian3d::PhysicsPlugins;
use bevy::app::App;
use bevy::asset::AssetPlugin;
use bevy::gltf::GltfPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy_persistent::Persistent;
use bevy_skein::SkeinPlugin;
use bevy_state::app::StatesPlugin;
use bevy_world_serialization::WorldAsset;
use tempfile::TempDir;

use belladonna_sherbet::plugins::camera::CameraPlugin;
use belladonna_sherbet::plugins::player::{KeyboardKeyBindings, PlayerPlugin};
use belladonna_sherbet::plugins::asset_management::AssetManagerPlugin;
use belladonna_sherbet::plugins::location_change::LocationChangePlugin;
use belladonna_sherbet::plugins::save::{SaveData, SavePlugin};
use belladonna_sherbet::plugins::game::{DataPath, GameState};
use belladonna_sherbet::plugins::player::PlayerSpawnState;

const TEST_YARN: &str = r#"
title: TestDialogue
---
TestNode:
    <<Test dialogue line>>
    ===
    -> END
"#;

pub struct TestApp {
    pub app: App,
    pub temp_dir: Option<TempDir>,
}

#[allow(dead_code)]
impl TestApp {
    pub fn new() -> Self {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let data_path = temp_dir.path().join("Coffee Constellations/Belladonna Sherbet");
        std::fs::create_dir_all(&data_path).expect("Failed to create data path");

        let config_dir = data_path.join("Config");
        std::fs::create_dir_all(&config_dir).expect("Failed to create config dir");

        let saves_dir = data_path.join("Saves");
        std::fs::create_dir_all(&saves_dir).expect("Failed to create saves dir");

        // Copy test yarn file to asset directory
        let yarn_src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/test_assets/test_dialogue.yarn");
        let yarn_dst = temp_dir.path().join("test_dialogue.yarn");
        if yarn_src.exists() {
            std::fs::copy(&yarn_src, &yarn_dst).expect("Failed to copy test yarn file");
        } else {
            std::fs::write(&yarn_dst, TEST_YARN).expect("Failed to write test yarn file");
        }

        let mut app = App::new();

        app.add_plugins((
            MinimalPlugins,
            AssetPlugin {
                file_path: temp_dir.path().to_string_lossy().to_string(),
                ..default()
            },
            LogPlugin::default(),
            GltfPlugin::default(),
            RenderPlugin::default(),
            PhysicsPlugins::default(),
            SkeinPlugin::default(),
            StatesPlugin,
        ))
        .add_plugins((
            CameraPlugin,
            PlayerPlugin,
            AssetManagerPlugin,
            LocationChangePlugin,
            SavePlugin,
        ));

        app.init_asset::<WorldAsset>();
        app.init_asset::<Mesh>();

        app.insert_resource(DataPath { path: data_path.clone() })
            .init_state::<GameState>()
            .init_state::<PlayerSpawnState>()
            .insert_resource(
                Persistent::<KeyboardKeyBindings>::builder()
                    .name("keyboard key bindings")
                    .format(bevy_persistent::StorageFormat::TomlPretty)
                    .path(config_dir.join("keyboard-keybindings.toml"))
                    .default(KeyboardKeyBindings {
                        up: [KeyCode::KeyW, KeyCode::ArrowUp],
                        down: [KeyCode::KeyS, KeyCode::ArrowDown],
                        left: [KeyCode::KeyA, KeyCode::ArrowLeft],
                        right: [KeyCode::KeyD, KeyCode::ArrowRight],
                    })
                    .revertible(true)
                    .revert_to_default_on_deserialization_errors(true)
                    .build()
                    .expect("failed to init keyboard keybindings")
            )
            .insert_resource(
                Persistent::<SaveData>::builder()
                    .name("Save Data")
                    .format(bevy_persistent::StorageFormat::TomlPretty)
                    .path(saves_dir.join("save.toml"))
                    .default(SaveData::default())
                    .build()
                    .expect("failed to init save file")
            );

        Self { app, temp_dir: Some(temp_dir) }
    }

    pub fn advance_frames(&mut self, frames: u32) {
        for _ in 0..frames {
            self.app.update();
            self.app.world_mut().run_schedule(FixedUpdate);
        }
    }

    pub fn press_key(&mut self, key: KeyCode) {
        let mut input = self.app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(key);
    }

    pub fn release_key(&mut self, key: KeyCode) {
        let mut input = self.app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.release(key);
    }

    pub fn spawn_player(&mut self) -> Entity {
        let mut commands = self.app.world_mut().commands();
        let entity = commands.spawn((
            belladonna_sherbet::plugins::player::PlayerSpawn,
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        )).id();
        self.advance_frames(2);
        entity
    }

    pub fn get_player_transform(&mut self) -> Option<Transform> {
        let mut query = self.app.world_mut().query_filtered::<&Transform, With<belladonna_sherbet::plugins::player::Player>>();
        query.single(self.app.world()).ok().copied()
    }

    pub fn get_camera_transform(&mut self) -> Option<Transform> {
        let mut query = self.app.world_mut().query_filtered::<&Transform, With<belladonna_sherbet::plugins::camera::PlayerCamera>>();
        query.single(self.app.world()).ok().copied()
    }

    pub fn get_game_state(&self) -> GameState {
        self.app.world().resource::<State<GameState>>().get().clone()
    }

    pub fn get_player_spawn_state(&self) -> PlayerSpawnState {
        self.app.world().resource::<State<PlayerSpawnState>>().get().clone()
    }

    pub fn get_location(&self) -> belladonna_sherbet::plugins::location_change::Location {
        self.app.world().resource::<State<belladonna_sherbet::plugins::location_change::Location>>().get().clone()
    }

    pub fn get_dialogue_state(&self) -> belladonna_sherbet::plugins::yarn::DialogueState {
        self.app.world().resource::<State<belladonna_sherbet::plugins::yarn::DialogueState>>().get().clone()
    }

    pub fn get_loading_state(&self) -> belladonna_sherbet::plugins::asset_management::LoadingState {
        self.app.world().resource::<State<belladonna_sherbet::plugins::asset_management::LoadingState>>().get().clone()
    }

    /// Reset the app state for the next test
    pub fn reset(&mut self) {
        // Clear all entities
        let entities: Vec<_> = self.app.world_mut().query::<Entity>().iter(self.app.world()).collect();
        for entity in entities {
            self.app.world_mut().despawn(entity);
        }
        // Reset states
        self.app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::InGame);
        self.app.world_mut().resource_mut::<NextState<PlayerSpawnState>>().set(PlayerSpawnState::NotYetSpawned);
        self.app.world_mut().resource_mut::<NextState<belladonna_sherbet::plugins::location_change::Location>>().set(belladonna_sherbet::plugins::location_change::Location::Test);
        self.app.world_mut().resource_mut::<NextState<belladonna_sherbet::plugins::yarn::DialogueState>>().set(belladonna_sherbet::plugins::yarn::DialogueState::Game);
        self.app.world_mut().resource_mut::<NextState<belladonna_sherbet::plugins::asset_management::LoadingState>>().set(belladonna_sherbet::plugins::asset_management::LoadingState::LevelReady);
        self.advance_frames(2);
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if let Some(temp_dir) = self.temp_dir.take() {
            temp_dir.close().ok();
        }
    }
}

pub fn setup_test_yarn_project(_app: &mut App, _yarn_path: &PathBuf) {
    // YarnProject is loaded automatically by YarnSpinnerPlugin from the asset path
}