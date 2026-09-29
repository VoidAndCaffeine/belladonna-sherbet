use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_persistent::{Persistent, StorageFormat};
use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;
use crate::plugins::game::DataPath;

pub struct SavePlugin;
impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup,setup)
            .add_systems(Update,quicksave.run_if(input_just_pressed(KeyCode::F5)))
        ;
    }
}

#[skip_serializing_none]
#[derive(Resource, Serialize, Deserialize)]
#[serde(default)]
pub struct SaveData{
    pub version: Option<String>,
    pub test: Option<i32>,
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData {
            version: None,
            test: None,
        }
    }
}

fn setup(
    mut commands: Commands,
    data_path: Res<DataPath>

) {
    let save_dir = data_path.path.join("Saves");
    commands.insert_resource(
        Persistent::<SaveData>::builder()
            .name("Save Data")
            .format(StorageFormat::TomlPretty)
            .path(save_dir.join("save.toml"))
            .default(SaveData::default())
            .build()
            .expect("failed to init save file")
    )
}

fn quicksave(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut save_data: ResMut<Persistent<SaveData>>,
) {
    if !keyboard.any_pressed([KeyCode::F5]) {return}
    save_data.version = Some(env!("CARGO_PKG_VERSION").to_string());
    match save_data.test {
        Some(test) => save_data.test = Some(test + 1),
        None => save_data.test = Some(1)
    }
    save_data.persist().expect("failed to save");
    info!("Game Saved");
    //ToDo: allow save data roll backs
}
