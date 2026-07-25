use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

pub struct YarnPlugin;
impl Plugin for YarnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                YarnSpinnerPlugin::new(),
                ExampleYarnSpinnerDialogueViewPlugin::new()
            ))
            .add_systems(Update, spawn_dialogue_runner.run_if(resource_added::<YarnProject>))
        ;
    }
}
fn spawn_dialogue_runner(
    mut commands: Commands,
    project: Res<YarnProject>,
){
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}