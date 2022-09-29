use crate::{prelude::*, GameElement};
use bevy::prelude::*;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct Logger {
    fragments: Vec<String>,
}

impl Logger {
    pub fn new() -> Self {
        Logger { fragments: Vec::new() }
    }

    pub fn append<T: ToString>(mut self, text: T) -> Self {
        self.fragments.push(text.to_string());
        self
    }

    pub fn to_str(&self) -> String {
        use std::fmt::Write;
        self.fragments.iter().enumerate().fold("\n".to_string(), |mut acc, (i, frag)| {
            if i == 0 {
                acc.push_str(frag);
            } else {
                write!(acc, " {}", frag).expect("write to string failed");
            }

            acc
        })
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

const NUM_LINES: usize = 6;

pub struct Console {
    dirty: Mutex<bool>,
    text: Mutex<Vec<(String, Color)>>,
}

impl Default for Console {
    fn default() -> Self {
        Console::new()
    }
}

impl Console {
    pub fn new() -> Self {
        let mut text = vec![(String::new(), Color::WHITE); NUM_LINES];
        text[0] = ("Welcome to Mega-Chicken".to_string(), Color::YELLOW);
        text[1] = (
            "Use cursor keys to move, J to jump, SPACE to interact with the object you are facing."
                .to_string(),
            Color::CYAN,
        );
        text[2] = (
            "Henry the dog is here to help you find the golden egg. He promises not to eat anything important."
                .to_string(),
            Color::GREEN,
        );
        Self { text: Mutex::new(text), dirty: Mutex::new(true) }
    }

    pub fn write<S: ToString>(&self, text: S) {
        self.write_color(text, Color::WHITE);
    }

    pub fn write_color<S: ToString>(&self, text: S, color: Color) {
        let mut text_lock = self.text.lock().unwrap();
        for i in (1..NUM_LINES).rev() {
            text_lock[i] = text_lock[i - 1].clone();
        }
        text_lock[0] = (text.to_string(), color);
        let mut dirty_lock = self.dirty.lock().unwrap();
        *dirty_lock = true;
    }

    #[allow(dead_code)]
    pub fn write_logger(&self, logger: Logger) {
        let text = logger.to_str();
        self.write_color(text, Color::WHITE);
    }

    fn is_dirty(&self) -> bool {
        let dirty_lock = self.dirty.lock().unwrap();
        *dirty_lock
    }

    fn clean(&self) {
        let mut dirty_lock = self.dirty.lock().unwrap();
        *dirty_lock = false;
    }
}

#[derive(Component)]
pub struct ConsoleLine(usize);

pub fn console_setup(mut commands: Commands, fonts: Res<FontAssets>, console: Res<Console>) {
    const FONT_SIZE: f32 = 18.0;

    let text_lock = console.text.lock().unwrap();
    for (i, (line, color)) in text_lock.iter().enumerate() {
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(4.0),
                        bottom: Val::Px(110.0 - (i as f32 * (FONT_SIZE + 1.0))),
                        ..default()
                    },
                    ..default()
                },
                text: Text::from_section(
                    line.clone(),
                    TextStyle { font: fonts.fira_sans.clone(), font_size: FONT_SIZE, color: *color },
                )
                .with_alignment(TextAlignment::CENTER_LEFT),
                ..default()
            })
            .insert(ConsoleLine(i))
            .insert(GameElement);
    }
}

pub fn update_consoles(mut query: Query<(&ConsoleLine, &mut Text)>, console: Res<Console>) {
    if console.is_dirty() {
        let line_lock = console.text.lock().unwrap();
        for (line, mut text) in query.iter_mut() {
            text.sections[0].value = line_lock[line.0].0.clone();
            text.sections[0].style.color = line_lock[line.0].1;
        }
        console.clean();
    }
}

pub struct ConsolePlugin;
impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Console>()
            .add_enter_system(AppState::Playing, console_setup)
            .add_system(update_consoles.run_in_state(AppState::Playing));
    }
}
