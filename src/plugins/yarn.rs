use avian3d::prelude::CollisionStart;
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;
use crate::plugins::player::Player;

pub struct YarnPlugin;
impl Plugin for YarnPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<YarnNode>()
            .init_state::<DialogueState>()
            .add_plugins((
                YarnSpinnerPlugin::new(),
                ExampleYarnSpinnerDialogueViewPlugin::new()
            ))
            .add_observer(spawn_dialogue_runner)
        ;
    }
}
#[derive(States,Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DialogueState{
    #[default]
    Game,
    Dialogue,
}
#[derive(Component,Reflect,Eq, PartialEq, Clone)]
#[reflect(Component)]
pub struct YarnNode {
    pub yarn_node: String,
    pub prompt: String,
}

impl YarnNode {
    pub fn new(yarn_node: impl Into<String>) -> Self {
        Self {
            yarn_node: yarn_node.into(),
            ..default()
        }
    }
}

impl Default for YarnNode {
    fn default() -> Self {
        Self {
            yarn_node: "".to_string(),
            prompt: "Talk".to_string(),
        }
    }
}

fn spawn_dialogue_runner(
    event: On<CollisionStart>,
    query_yarn: Query<&YarnNode>,
    query_player: Query<&Player>,
    mut commands: Commands,
    project: Res<YarnProject>,
){
    let yarn_node = match query_yarn.get(event.event_target()) {
        Ok(yarn_node) => {
            yarn_node
        },
        Err(_) => return,
    };

    let _ = match query_player.get(event.collider2) {
        Ok(_) => {info!("Player entered yarn node {}", yarn_node.yarn_node)},
        Err(_) => return,
    };

    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner.start_node(yarn_node.yarn_node.clone());
    commands.spawn(dialogue_runner);
}