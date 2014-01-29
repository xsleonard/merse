use rsfml::system::vector2::Vector2i;
use dungeon::Floor;
use std::num::{max, min};

pub struct Position {
    p: Vector2i,
}

impl Position {
    pub fn move(&mut self, floor: &Floor, delta_x: i32, delta_y: i32) {
        self.p.x += delta_x;
        self.p.y += delta_y;
        self.p.x = max(0, min(self.p.x, floor.dim.x - 1));
        self.p.y = max(0, min(self.p.y, floor.dim.y - 1));
    }
}
