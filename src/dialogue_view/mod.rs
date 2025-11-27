use bevy::prelude::*;
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;

mod assets;
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
        .add_plugins(option_selection::option_selection_plugin);
}
