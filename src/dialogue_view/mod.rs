use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;

pub(crate) mod assets;
mod history;
mod option_selection;
mod setup;
mod typewriter;
mod updating;

/// The [`SystemSet`] containing all systems added by the [`UndertowYarnSpinnerDialogueViewPlugin`].
/// Is run after the [`YarnSpinnerSystemSet`](bevy_yarnspinner::prelude::YarnSpinnerSystemSet).
#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct YarnSpinnerDialogueViewSystemSet;

pub(super) fn plugin(app: &mut App) {
    assert!(
        app.is_plugin_added::<YarnSpinnerPlugin>(),
        "YarnSpinnerPlugin must be added before UndertowYarnSpinnerDialogueViewPlugin"
    );
    app.add_plugins(assets::ui_assets_plugin)
        .add_plugins(setup::ui_setup_plugin)
        .add_plugins(updating::ui_updating_plugin)
        .add_plugins(typewriter::typewriter_plugin)
        .add_plugins(option_selection::option_selection_plugin)
        .add_plugins(history::plugin);
}

pub fn cleanup_system(
    mut commands: Commands,
    mut root_visibility: Single<&mut Visibility, With<setup::UiRootNode>>,
) {
    // 1. Remove the typewriter state so it doesn't resume typing old text next time
    commands.remove_resource::<typewriter::Typewriter>();

    // 2. Remove any pending options
    commands.remove_resource::<option_selection::OptionSelection>();

    // 3. Force hide the UI root immediately
    **root_visibility = Visibility::Hidden;
}
