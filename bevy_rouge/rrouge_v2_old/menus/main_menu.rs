use crate::menus::*;

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct GameOverUI;

fn splash_screen(mut commands: Commands, fonts: Res<FontAssets>, turn_state: Res<CurrentState<AppState>>) {
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
            // chose title based on State
            let mut title = "";
            let mut title_color = Color::GOLD;

            if turn_state.0 == AppState::MainMenu {
                title = "Beby RRogue";
            } else if turn_state.0 == AppState::GameOver {
                title = "Game Over";
                title_color = Color::RED;
            }

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
                            value: title.to_string(),
                            style: TextStyle {
                                font: fonts.fira_sans.clone(),
                                font_size: 100.0,
                                color: title_color,
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

pub fn splash_screen_input(
    mut commands: Commands,
    state: Res<CurrentState<AppState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key {
        // reset keyboard, bevys bug when changing states
        keyboard_input.reset(key);

        // update state
        let next_state =
            if state.0 == AppState::MainMenu { AppState::DungeonCrawlEnter } else { AppState::MainMenu };

        commands.insert_resource(NextState(next_state));
    }
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::MainMenu, splash_screen)
            .add_system(splash_screen_input.run_in_state(AppState::MainMenu))
            .add_exit_system(
                AppState::MainMenu,
                despawn_all_with::<MenuUI>.chain(despawn_all_with::<GameOverUI>),
            );
    }
}
