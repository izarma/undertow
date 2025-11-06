use bevy::{
    ecs::relationship::RelatedSpawner,
    input_focus::{
        InputDispatchPlugin,
        tab_navigation::{TabGroup, TabNavigationPlugin},
    },
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{
        ControlOrientation, CoreScrollbarDragState, CoreScrollbarThumb, Scrollbar, ScrollbarPlugin,
    },
};

use crate::menus::Menu;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ScrollbarPlugin, InputDispatchPlugin, TabNavigationPlugin));
    app.add_systems(OnEnter(Menu::SaveGame), setup_game_load_ui)
        .add_systems(
            Update,
            update_scrollbar_thumb.run_if(in_state(Menu::SaveGame)),
        );
}

fn setup_game_load_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            left: px(100),
            top: px(100),
            right: px(100),
            bottom: px(100),
            padding: UiRect::all(px(15)),
            row_gap: px(10),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.9, 0.1)),
        TabGroup::default(),
        Children::spawn((Spawn(Text::new("Load Game")), Spawn(load_scroller()))),
    ));
}

fn load_scroller() -> impl Bundle {
    (
        Node {
            display: Display::Grid,
            width: px(600),
            height: px(400),
            grid_template_columns: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
            grid_template_rows: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
            row_gap: px(2),
            column_gap: px(2),

            ..default()
        },
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            let scroll_area_id = parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_items: JustifyItems::Default,
                        padding: UiRect::all(px(50)),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.9)),
                    ScrollPosition(Vec2::new(0.0, 10.0)),
                    Children::spawn((
                        Spawn(save_row("blabla")),
                        Spawn(save_row("blabla2")),
                        Spawn(save_row("blabla3")),
                        Spawn(save_row("blabla4")),
                        Spawn(save_row("blabla5")),
                        Spawn(save_row("blabla6")),
                        Spawn(save_row("blabla7")),
                        Spawn(save_row("blabla8")),
                        Spawn(save_row("blabla9")),
                        Spawn(save_row("blabla10")),
                    )),
                ))
                .id();
            parent.spawn((
                Node {
                    min_width: px(8),
                    grid_row: GridPlacement::start(1),
                    grid_column: GridPlacement::start(2),
                    ..default()
                },
                Scrollbar {
                    orientation: ControlOrientation::Vertical,
                    target: scroll_area_id,
                    min_thumb_length: 8.0,
                },
                Children::spawn(Spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    Hovered::default(),
                    BackgroundColor(Color::srgb(0.9, 0.1, 0.1)),
                    BorderRadius::all(px(4)),
                    CoreScrollbarThumb,
                ))),
            ));
        })),
    )
}

/// Create a single save row
fn save_row(caption: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Percent(100.0),
            min_height: Val::Px(100.0), // Each button is 100px tall (3 * 100 = 300px scroll area)
            display: Display::Flex,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(px(10)),
            margin: UiRect::bottom(px(8)), // Space between buttons
            ..default()
        },
        BackgroundColor(Color::srgb(0.9, 0.9, 0.1)),
        BorderRadius::all(px(8)),
        Interaction::None,
        Children::spawn(Spawn((
            Text::new(caption),
            TextFont {
                font_size: 16.0,
                ..default()
            },
        ))),
    )
}

// Update the color of the scrollbar thumb.
fn update_scrollbar_thumb(
    mut q_thumb: Query<
        (&mut BackgroundColor, &Hovered, &CoreScrollbarDragState),
        (
            With<CoreScrollbarThumb>,
            Or<(Changed<Hovered>, Changed<CoreScrollbarDragState>)>,
        ),
    >,
) {
    for (mut thumb_bg, Hovered(is_hovering), drag) in q_thumb.iter_mut() {
        let color: Color = if *is_hovering || drag.dragging {
            // If hovering, use a lighter color
            Color::srgb(0.7, 0.1, 0.1)
        } else {
            // Default color for the slider
            Color::srgb(0.9, 0.1, 0.1)
        }
        .into();

        if thumb_bg.0 != color {
            // Update the color of the thumb
            thumb_bg.0 = color;
        }
    }
}
