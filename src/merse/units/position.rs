use rsfml::system::vector2::Vector2i;
use dungeon::Dungeon;
use std::num::{max, min, abs};

pub struct Position {
    p: Vector2i,
    floor: uint,
}

impl Position {
    pub fn move(&mut self, dungeon: &Dungeon, delta_x: i32, delta_y: i32) {
        let floor = dungeon.current_floor();
        let mut p = self.p.clone();
        if delta_x != 0 {
            let dx = delta_x / abs(delta_x);
            for _ in range(0, abs(delta_x)) {
                p.x += dx;
                if dungeon.sprite(floor.tile_at(p).sprite).solid {
                    p.x -= dx;
                    break;
                }
            }
        }
        let mut q = self.p.clone();
        if delta_y != 0 {
            let dy = delta_y / abs(delta_y);
            for _ in range(0, abs(delta_y)) {
                q.y += dy;
                if dungeon.sprite(floor.tile_at(q).sprite).solid {
                    q.y -= dy;
                    break;
                }
            }
        }

        self.p.x = max(0, min(p.x, floor.dim.x - 1));
        self.p.y = max(0, min(q.y, floor.dim.y - 1));
    }

    pub fn ascend(&mut self, dungeon: &Dungeon) {
        match dungeon.floor_above() {
            Some(floor) => {
                if self.p == floor.downstairs {
                    self.floor += 1;
                }
            }
            None => (),
        };
    }

    pub fn descend(&mut self, dungeon: &Dungeon) {
        print!("Descending...\n");
        if self.p == dungeon.current_floor().downstairs {
            self.floor -= 1;
        }
    }
}
