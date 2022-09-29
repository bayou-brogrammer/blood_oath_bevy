use bevy::prelude::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Logger {
    color: Color,
    pub(super) fragments: Vec<String>,
}

impl Logger {
    pub fn new() -> Self { Logger { color: Color::WHITE, fragments: Vec::new() } }

    pub fn new_with_color<C: Into<Color>>(color: C) -> Self {
        Logger { color: color.into(), fragments: Vec::new() }
    }

    pub fn append<T: ToString>(mut self, text: T) -> Self {
        self.fragments.push(text.to_string());
        self
    }

    pub fn get_fragments(&self) -> (String, Color) {
        let log_text = self.fragments.iter().enumerate().fold("\n".to_string(), |mut acc, (i, frag)| {
            if i == 0 {
                acc.push_str(frag);
            } else {
                acc.push_str(&format!(" {}", frag));
            }
            acc
        });

        (log_text, self.color)
    }
}
