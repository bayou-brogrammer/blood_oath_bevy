use crate::prelude::*;

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct GameOverUI;

fn splash_screen(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    turn_state: Res<CurrentState<AppState>>,
    top_ui_node_q: Query<Entity, With<TopUINode>>,
) {
    // If we are not in MainMenu we need to remove ALL the other UI stuff around the game
    if turn_state.0 != AppState::MainMenu {
        let top_ui_node = top_ui_node_q.single();
        commands.entity(top_ui_node).despawn_recursive();
    }

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: UiColor(Color::rgb(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(MenuUI)
        .with_children(|parent| {
            // Spawn menu text
            parent.spawn_bundle(TextBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(140. * 1.)),
                    margin: UiRect { left: Val::Auto, right: Val::Auto, bottom: Val::Auto, top: Val::Auto },
                    ..Default::default()
                },
                // Use `Text` directly
                text: Text {
                    // Construct a `Vec` of `TextSection`s
                    sections: vec![
                        TextSection {
                            value: "Blood Oath".to_string(),
                            style: TextStyle {
                                font: fonts.fira_sans.clone(),
                                font_size: 100.0,
                                color: Color::RED,
                            },
                        },
                        TextSection {
                            value: "\nPress any key to start game.".to_string(),
                            style: TextStyle {
                                font: fonts.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    alignment: TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                },
                ..Default::default()
            });
        });
}

pub fn splash_screen_input(mut commands: Commands, mut keyboard_input: ResMut<Input<KeyCode>>) {
    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key {
        // reset keyboard, bevys bug when changing states
        keyboard_input.reset(key);
        commands.insert_resource(NextState(AppState::MapGen));
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::MainMenu, splash_screen)
            .add_system(splash_screen_input.run_in_state(AppState::MainMenu))
            .add_exit_system(
                AppState::MainMenu,
                despawn_all_with::<MenuUI>.chain(despawn_all_with::<GameOverUI>),
            );

        // setup when entering the gameover screen
        app.add_enter_system(AppState::GameOver, splash_screen)
            .add_system(splash_screen_input.run_in_state(AppState::GameOver))
            .add_exit_system(
                AppState::GameOver,
                despawn_all_with::<MenuUI>.chain(despawn_all_with::<GameOverUI>),
            );

        // setup when entering the victory screen
        app.add_enter_system(AppState::Victory, splash_screen)
            .add_system(splash_screen_input.run_in_state(AppState::Victory))
            .add_exit_system(
                AppState::Victory,
                despawn_all_with::<MenuUI>.chain(despawn_all_with::<GameOverUI>),
            );

        app.add_enter_system(AppState::NextLevel, splash_screen)
            .add_system(splash_screen_input.run_in_state(AppState::NextLevel))
            .add_exit_system(
                AppState::NextLevel,
                despawn_all_with::<MenuUI>.chain(despawn_all_with::<GameOverUI>),
            );
    }
}
