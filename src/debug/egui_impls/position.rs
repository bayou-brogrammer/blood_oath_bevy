use super::*;
use bevy_inspector_egui::options::NumberAttributes;

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
            ui.style_mut().spacing.item_spacing =
                bevy_inspector_egui::egui::Vec2::new(4.0, 0.);

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
