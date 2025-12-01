//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{
    MenuAssets,
    asset_tracking::{LoadResource, ResourceHandles},
    menus::{GameEnded, Menu},
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<MenuAssets>();
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    app.init_resource::<GameEnded>();
}

fn spawn_main_menu(mut commands: Commands, ended: Res<GameEnded>, asset_server: Res<AssetServer>) {
    if ended.0 {
        commands.spawn((
            DespawnOnEnter(Screen::Gameplay),
            ImageNode {
                image: asset_server.load("images/scenes/menu-theme2.png"),
                ..default()
            },
        ));
    } else {
        commands.spawn((
            DespawnOnEnter(Screen::Gameplay),
            ImageNode {
                image: asset_server.load("images/scenes/menu-theme.png"),
                ..default()
            },
        ));
    }

    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::header("Undertow", asset_server.load("dialogue/Granite-Bgvl.ttf")),
            widget::button("New Game", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::header("Undertow", asset_server.load("dialogue/Granite-Bgvl.ttf")),
            widget::button("Start", enter_loading_or_gameplay_screen),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

pub(crate) fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
