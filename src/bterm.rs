use crate::prelude::*;

pub fn setup_bterm() -> BTermBuilder {
    BTermBuilder::empty()
        .with_random_number_generator(true)
        .with_ortho_camera(false)
        .with_font("terminal16x24.png", 16, 16, (16.0, 24.0))
        .with_font("dungeonfontv2.png", 16, 16, (32.0, 32.0))
        .with_font("terminal12x12.png", 16, 16, (12.0, 12.0))
        .with_font("vga8x16.png", 16, 16, (8.0, 16.0))
        .with_simple_console(2, DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_background(true)
        .with_simple_console(2, DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_background(false)
}
