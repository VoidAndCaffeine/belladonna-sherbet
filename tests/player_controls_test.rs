use bevy::prelude::*;
use bevy_state::app::StatesPlugin;
use bevy_persistent::Persistent;
use belladonna_sherbet::plugins::player::{apply_controls, ControlScheme, KeyboardKeyBindings};
use belladonna_sherbet::plugins::yarn::DialogueState;
use belladonna_sherbet::plugins::game::GameState;
use bevy_tnua::prelude::*;
use tempfile::TempDir;

fn setup_test_app() -> (App, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("Config");
    std::fs::create_dir_all(&config_dir).unwrap();

    let mut app = App::new();
    app.add_plugins(StatesPlugin)
        .init_resource::<ButtonInput<KeyCode>>()
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
        .init_state::<GameState>()
        .init_state::<DialogueState>()
        .add_systems(FixedUpdate, apply_controls.in_set(TnuaUserControlsSystems));
    app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::InGame);
    app.world_mut().resource_mut::<NextState<DialogueState>>().set(DialogueState::Game);
    app.update();
    (app, temp_dir)
}

fn run_fixed_update(app: &mut App) {
    app.world_mut().run_schedule(FixedUpdate);
}

#[test]
fn test_apply_controls_no_input() {
    let (mut app, _temp_dir) = setup_test_app();
    let controller = TnuaController::<ControlScheme>::default();
    let entity = app.world_mut().spawn(controller).id();

    run_fixed_update(&mut app);

    let controller = app.world().get::<TnuaController<ControlScheme>>(entity).unwrap();
    // basis is TnuaBuiltinWalk directly, access desired_motion field
    assert_eq!(controller.basis.desired_motion, Vec3::ZERO);
}

#[test]
fn test_apply_controls_forward() {
    let (mut app, _temp_dir) = setup_test_app();
    let controller = TnuaController::<ControlScheme>::default();
    let entity = app.world_mut().spawn(controller).id();

    press_key(&mut app, KeyCode::KeyW);
    run_fixed_update(&mut app);

    let controller = app.world().get::<TnuaController<ControlScheme>>(entity).unwrap();
    assert!(controller.basis.desired_motion.z < -0.1, "Should move forward (negative Z)");
    assert!(controller.basis.desired_motion.x.abs() < 0.1);
}

#[test]
fn test_apply_controls_backward() {
    let (mut app, _temp_dir) = setup_test_app();
    let controller = TnuaController::<ControlScheme>::default();
    let entity = app.world_mut().spawn(controller).id();

    press_key(&mut app, KeyCode::KeyS);
    run_fixed_update(&mut app);

    let controller = app.world().get::<TnuaController<ControlScheme>>(entity).unwrap();
    assert!(controller.basis.desired_motion.z > 0.1, "Should move backward (positive Z)");
}

#[test]
fn test_apply_controls_left() {
    let (mut app, _temp_dir) = setup_test_app();
    let controller = TnuaController::<ControlScheme>::default();
    let entity = app.world_mut().spawn(controller).id();

    press_key(&mut app, KeyCode::KeyA);
    run_fixed_update(&mut app);

    let controller = app.world().get::<TnuaController<ControlScheme>>(entity).unwrap();
    assert!(controller.basis.desired_motion.x < -0.1, "Should move left (negative X)");
}

#[test]
fn test_apply_controls_right() {
    let (mut app, _temp_dir) = setup_test_app();
    let controller = TnuaController::<ControlScheme>::default();
    let entity = app.world_mut().spawn(controller).id();

    press_key(&mut app, KeyCode::KeyD);
    run_fixed_update(&mut app);

    let controller = app.world().get::<TnuaController<ControlScheme>>(entity).unwrap();
    assert!(controller.basis.desired_motion.x > 0.1, "Should move right (positive X)");
}

#[test]
fn test_apply_controls_diagonal_normalized() {
    let (mut app, _temp_dir) = setup_test_app();
    let controller = TnuaController::<ControlScheme>::default();
    let entity = app.world_mut().spawn(controller).id();

    press_key(&mut app, KeyCode::KeyW);
    press_key(&mut app, KeyCode::KeyD);
    run_fixed_update(&mut app);

    let controller = app.world().get::<TnuaController<ControlScheme>>(entity).unwrap();
    let len = controller.basis.desired_motion.length();
    assert!((len - 1.0).abs() < 0.1, "Diagonal movement should be normalized to 1.0, got {}", len);
}

#[test]
fn test_apply_controls_opposing_keys_cancel() {
    let (mut app, _temp_dir) = setup_test_app();
    let controller = TnuaController::<ControlScheme>::default();
    let entity = app.world_mut().spawn(controller).id();

    press_key(&mut app, KeyCode::KeyW);
    press_key(&mut app, KeyCode::KeyS);
    run_fixed_update(&mut app);

    let controller = app.world().get::<TnuaController<ControlScheme>>(entity).unwrap();
    assert!(controller.basis.desired_motion.z.abs() < 0.1, "Opposing keys should cancel");
}

#[test]
fn test_apply_controls_arrow_keys() {
    let (mut app, _temp_dir) = setup_test_app();
    let controller = TnuaController::<ControlScheme>::default();
    let entity = app.world_mut().spawn(controller).id();

    press_key(&mut app, KeyCode::ArrowUp);
    run_fixed_update(&mut app);

    let controller = app.world().get::<TnuaController<ControlScheme>>(entity).unwrap();
    assert!(controller.basis.desired_motion.z < -0.1, "ArrowUp should move forward");
}

fn press_key(app: &mut App, key: KeyCode) {
    let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
    input.press(key);
}