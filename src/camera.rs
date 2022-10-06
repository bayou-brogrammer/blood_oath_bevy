use crate::prelude::*;

pub const VIEWPORT_X_OFFSET: i32 = 20;
pub const VIEWPORT_Y_OFFSET: i32 = 10;
pub const VIEWPORT_WIDTH: i32 = 100;
pub const VIEWPORT_HEIGHT: i32 = 100;

#[derive(Debug, Copy, Clone)]
pub struct CameraView {
    pub viewport: Rect,
    pub player_pos: Coord,
}

impl CameraView {
    pub fn new(player_pos: Coord) -> Self {
        Self {
            viewport: Rect::with_size(
                player_pos.x - VIEWPORT_X_OFFSET,
                player_pos.y - VIEWPORT_Y_OFFSET,
                VIEWPORT_WIDTH,
                VIEWPORT_HEIGHT,
            ),
            player_pos,
        }
    }

    fn calculate_viewport(&self) -> Rect {
        Rect::with_size(
            self.player_pos.x - VIEWPORT_X_OFFSET,
            self.player_pos.y - VIEWPORT_Y_OFFSET,
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
        )
    }

    pub fn on_player_move(&mut self, player_pos: Coord) {
        self.player_pos = player_pos;
        self.viewport = self.calculate_viewport();
    }

    pub fn world_to_screen(&self, pt: Point) -> Point {
        let bot = pt - self.player_pos.to_point();
        bot + Point::new(VIEWPORT_X_OFFSET, VIEWPORT_Y_OFFSET)
    }

    pub fn world_to_screen_text(&self, pt: Point) -> Point {
        let ws = self.world_to_screen(pt);
        ws * Point::new(2, 1)
    }

    pub fn screen_to_world(&self, mouse_pt: Point) -> Point {
        Point::new(mouse_pt.x + self.viewport.x1, mouse_pt.y + self.viewport.y1)
    }
}
