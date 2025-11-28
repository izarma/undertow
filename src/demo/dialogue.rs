use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnFileSource, YarnProject, YarnSpinnerPlugin};

use crate::{
    Pause,
    demo::level::{LevelAssets, SceneBackground},
    dialogue_view::{self, YarnSpinnerDialogueViewSystemSet},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file(
        "dialogue/undertow.yarn",
    )),));
    app.configure_sets(
        Update,
        YarnSpinnerDialogueViewSystemSet.run_if(in_state(Pause(false))),
    );
    app.add_systems(
        OnEnter(Screen::Gameplay),
        // Spawn the dialogue runner once the Yarn project has finished compiling
        spawn_dialogue_runner.run_if(resource_exists::<YarnProject>),
    );
    app.add_systems(OnExit(Screen::Gameplay), dialogue_view::cleanup_system);
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
    dialogue_runner
        .commands_mut()
        .add_command("next_scene", commands.register_system(next_scene));
    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node("Shore");
    commands.spawn((dialogue_runner, DespawnOnExit(Screen::Gameplay)));
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
