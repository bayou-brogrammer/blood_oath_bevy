use crate::egui::Vec2;
use crate::prelude::*;
use bevy_inspector_egui::{options::NumberAttributes, Inspectable};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Component)]
pub struct Position(pub Point);

impl Default for Position {
    fn default() -> Self {
        Self(Point::zero())
    }
}

impl Deref for Position {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Point> for Position {
    fn from(pt: Point) -> Self {
        Position(pt)
    }
}

impl Inspectable for Position {
    type Attributes = NumberAttributes<i32>;

    fn ui(
        &mut self,
        ui: &mut bevy_inspector_egui::egui::Ui,
        options: Self::Attributes,
        context: &mut bevy_inspector_egui::Context,
    ) -> bool {
        let mut changed = false;

        ui.scope(|ui| {
            ui.style_mut().spacing.item_spacing = Vec2::new(4.0, 0.);

            ui.columns(2, |ui| {
                let x_attrs = NumberAttributes {
                    min: options.min,
                    max: options.max,
                    speed: options.speed,
                    ..Default::default()
                };

                let y_attrs = NumberAttributes {
                    min: options.min,
                    max: options.max,
                    speed: options.speed,
                    ..Default::default()
                };
                changed |= self.x.ui(&mut ui[0], x_attrs, context);
                changed |= self.y.ui(&mut ui[1], y_attrs, context);
            });
        });

        changed
    }
}
