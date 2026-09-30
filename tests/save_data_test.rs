use serial_test::serial;
use belladonna_sherbet::plugins::save::SaveData;
use std::env;

#[test]
#[serial]
fn test_save_data_default() {
    let save_data = SaveData::default();
    assert_eq!(save_data.version, None);
    assert_eq!(save_data.test, None);
}

#[test]
#[serial]
fn test_save_data_serialization() {
    let mut save_data = SaveData::default();
    save_data.version = Some("0.1.0".to_string());
    save_data.test = Some(42);

    let toml = toml::to_string(&save_data).expect("Failed to serialize");
    let deserialized: SaveData = toml::from_str(&toml).expect("Failed to deserialize");

    assert_eq!(save_data.version, deserialized.version);
    assert_eq!(save_data.test, deserialized.test);
}

#[test]
#[serial]
fn test_save_data_version_field() {
    let mut save_data = SaveData::default();
    save_data.version = Some(env!("CARGO_PKG_VERSION").to_string());

    assert_eq!(save_data.version, Some(env!("CARGO_PKG_VERSION").to_string()));
}

#[test]
#[serial]
fn test_save_data_test_counter_increment() {
    let mut save_data = SaveData::default();

    save_data.test = Some(1);
    assert_eq!(save_data.test, Some(1));

    save_data.test = Some(save_data.test.unwrap() + 1);
    assert_eq!(save_data.test, Some(2));

    save_data.test = Some(save_data.test.unwrap() + 1);
    assert_eq!(save_data.test, Some(3));
}

#[test]
#[serial]
fn test_save_data_none_to_some() {
    let mut save_data = SaveData::default();
    assert_eq!(save_data.test, None);

    save_data.test = Some(1);
    assert_eq!(save_data.test, Some(1));
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

#[test]
#[serial]
fn test_save_data_with_some_fields() {
    let mut save_data = SaveData::default();
    save_data.version = Some("0.1.0".to_string());

    let toml_str = toml::to_string(&save_data).unwrap();
    assert!(toml_str.contains("version"));
    assert!(!toml_str.contains("test"));
}