use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnProject, YarnSpinnerPlugin};
use undertow_dialogue_view::UndertowYarnSpinnerDialogueViewPlugin;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        YarnSpinnerPlugin::new(),
        UndertowYarnSpinnerDialogueViewPlugin::new(),
    ));
    app.add_systems(
        OnEnter(Screen::Gameplay),
        // Spawn the dialogue runner once the Yarn project has finished compiling
        spawn_dialogue_runner.run_if(resource_added::<YarnProject>), //.run_if(in_state(Screen::Gameplay)),
    );
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node("Shore");
    commands.spawn(dialogue_runner);
    info!("Dialogue runner spawned");
}
