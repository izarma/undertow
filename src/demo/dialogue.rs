use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnProject, YarnSpinnerPlugin};
use undertow_dialogue_view::UndertowYarnSpinnerDialogueViewPlugin;

use crate::{
    demo::level::{LevelAssets, SceneBackground},
    screens::Screen,
};

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
    dialogue_runner
        .commands_mut()
        .add_command("next_scene", commands.register_system(next_scene));
    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node("Shore");
    commands.spawn(dialogue_runner);
    info!("Dialogue runner spawned");
}

fn next_scene(
    level_assets: Res<LevelAssets>,
    mut scene_query: Query<(&mut Sprite, &mut SceneBackground)>,
) {
    if let Ok((mut sprite, mut scene_bg)) = scene_query.single_mut() {
        scene_bg.index = (scene_bg.index + 1) % level_assets.scenes.len();
        sprite.image = level_assets.scenes[scene_bg.index].clone();
        info!("Changed to scene {}", scene_bg.index);
    }
}
