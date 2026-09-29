use belladonna_sherbet::plugins::player::KeyboardKeyBindings;
use bevy::prelude::*;

#[test]
fn test_keyboard_keybindings_default() {
    let bindings = KeyboardKeyBindings {
        up: [KeyCode::KeyW, KeyCode::ArrowUp],
        down: [KeyCode::KeyS, KeyCode::ArrowDown],
        left: [KeyCode::KeyA, KeyCode::ArrowLeft],
        right: [KeyCode::KeyD, KeyCode::ArrowRight],
    };

    assert_eq!(bindings.up, [KeyCode::KeyW, KeyCode::ArrowUp]);
    assert_eq!(bindings.down, [KeyCode::KeyS, KeyCode::ArrowDown]);
    assert_eq!(bindings.left, [KeyCode::KeyA, KeyCode::ArrowLeft]);
    assert_eq!(bindings.right, [KeyCode::KeyD, KeyCode::ArrowRight]);
}

#[test]
fn test_keyboard_keybindings_serialization() {
    let bindings = KeyboardKeyBindings {
        up: [KeyCode::KeyW, KeyCode::ArrowUp],
        down: [KeyCode::KeyS, KeyCode::ArrowDown],
        left: [KeyCode::KeyA, KeyCode::ArrowLeft],
        right: [KeyCode::KeyD, KeyCode::ArrowRight],
    };

    let toml = toml::to_string(&bindings).expect("Failed to serialize");
    let deserialized: KeyboardKeyBindings = toml::from_str(&toml).expect("Failed to deserialize");

    assert_eq!(bindings.up, deserialized.up);
    assert_eq!(bindings.down, deserialized.down);
    assert_eq!(bindings.left, deserialized.left);
    assert_eq!(bindings.right, deserialized.right);
}

#[test]
fn test_keyboard_keybindings_custom() {
    let bindings = KeyboardKeyBindings {
        up: [KeyCode::KeyI, KeyCode::ArrowUp],
        down: [KeyCode::KeyK, KeyCode::ArrowDown],
        left: [KeyCode::KeyJ, KeyCode::ArrowLeft],
        right: [KeyCode::KeyL, KeyCode::ArrowRight],
    };

    assert_eq!(bindings.up, [KeyCode::KeyI, KeyCode::ArrowUp]);
    assert_eq!(bindings.down, [KeyCode::KeyK, KeyCode::ArrowDown]);
    assert_eq!(bindings.left, [KeyCode::KeyJ, KeyCode::ArrowLeft]);
    assert_eq!(bindings.right, [KeyCode::KeyL, KeyCode::ArrowRight]);
}

#[test]
fn test_keyboard_keybindings_toml_roundtrip() {
    let original = KeyboardKeyBindings {
        up: [KeyCode::KeyW, KeyCode::ArrowUp],
        down: [KeyCode::KeyS, KeyCode::ArrowDown],
        left: [KeyCode::KeyA, KeyCode::ArrowLeft],
        right: [KeyCode::KeyD, KeyCode::ArrowRight],
    };

    let toml_str = toml::to_string_pretty(&original).unwrap();
    let parsed: KeyboardKeyBindings = toml::from_str(&toml_str).unwrap();

    assert_eq!(original.up, parsed.up);
    assert_eq!(original.down, parsed.down);
    assert_eq!(original.left, parsed.left);
    assert_eq!(original.right, parsed.right);
}