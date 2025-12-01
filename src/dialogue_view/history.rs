use bevy::{
    ecs::relationship::RelatedSpawner,
    input::common_conditions::input_just_pressed,
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

use crate::{
    dialogue_view::setup::text_style,
    menus::Menu,
    screens::gameplay,
    theme::{interaction::MenuAssets, widget},
};

#[derive(Resource, Default)]
pub struct DialogueHistory {
    pub lines: Vec<String>,
}

#[derive(Component)]
pub struct HistoryMenuRoot;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ScrollbarPlugin, InputDispatchPlugin, TabNavigationPlugin));

    // Systems for the new menu state
    app.add_systems(
        OnEnter(Menu::History),
        (spawn_history_menu, gameplay::pause),
    );
    app.add_systems(
        OnExit(Menu::History),
        (despawn_history_menu, gameplay::unpause),
    );
    app.add_systems(
        Update,
        // System to go back to the previous menu (Gameplay)
        go_back_to_gameplay
            .run_if(in_state(Menu::History).and(input_just_pressed(KeyCode::Escape))),
    );
    app.add_systems(
        Update,
        update_scrollbar_thumb.run_if(in_state(Menu::History)),
    );
    app.init_resource::<DialogueHistory>();
}

fn spawn_history_menu(
    mut commands: Commands,
    history: Res<DialogueHistory>,
    font: Res<MenuAssets>,
) {
    commands.spawn((
        GlobalZIndex(3),
        DespawnOnExit(Menu::History),
        HistoryMenuRoot,
        TabGroup::default(),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.0),
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.8)),
        Children::spawn(((
            Spawn(widget::header("History", font.menu_font.clone())),
            Spawn(load_scroller(history)),
            Spawn(widget::button("Back", go_back_to_gameplay_on_click)),
        ),)),
    ));
}

fn load_scroller(history: Res<DialogueHistory>) -> impl Bundle {
    let lines = history.lines.clone();
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
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            let mut scroll_area = parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect::all(px(50)),
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
                BackgroundColor(Color::BLACK.with_alpha(0.9)),
                ScrollPosition(Vec2::new(0.0, f32::MAX)),
            ));
            let scroll_area_id = scroll_area.id();

            // Spawn each history line as a child
            for line in lines.iter() {
                scroll_area.with_child(history_line(line));
            }
            parent.spawn((
                Node {
                    min_width: px(10),
                    grid_row: GridPlacement::start(1),
                    grid_column: GridPlacement::start(2),
                    ..default()
                },
                Scrollbar {
                    orientation: ControlOrientation::Vertical,
                    target: scroll_area_id,
                    min_thumb_length: 10.0,
                },
                Children::spawn(Spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    Hovered::default(),
                    BackgroundColor(Color::WHITE.with_alpha(0.9)),
                    BorderRadius::all(px(4)),
                    CoreScrollbarThumb,
                ))),
            ));
        })),
    )
}

/// Create a single history line
fn history_line(caption: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Percent(100.0),
            min_height: Val::Px(90.0),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            padding: UiRect::all(px(10)),
            margin: UiRect::bottom(px(8)), // Space between buttons
            ..default()
        },
        Interaction::None,
        Children::spawn(Spawn((Text::new(caption), text_style::standard()))),
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
            Color::WHITE.with_alpha(0.4)
        } else {
            // Default color for the slider
            Color::WHITE.with_alpha(0.9)
        }
        .into();

        if thumb_bg.0 != color {
            // Update the color of the thumb
            thumb_bg.0 = color;
        }
    }
}

fn go_back_to_gameplay(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn go_back_to_gameplay_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn despawn_history_menu(mut commands: Commands, query: Query<Entity, With<HistoryMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
