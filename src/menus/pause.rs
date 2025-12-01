//! The pause menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    MenuAssets,
    dialogue_view::history::DialogueHistory,
    menus::{Menu, main::open_settings_menu},
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands, font: Res<MenuAssets>) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            widget::header("Game paused", font.menu_font.clone()),
            widget::button("Continue", close_menu),
            widget::button("Settings", open_settings_menu),
            widget::button("Main Menu", quit_to_title),
        ],
    ));
}

fn close_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(
    _: On<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut history: ResMut<DialogueHistory>,
) {
    history.lines.clear();
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
