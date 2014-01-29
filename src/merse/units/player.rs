use rsfml::system::vector2::Vector2i;
use self::position::Position;
use dungeon::Tile;

mod position;

pub struct Player {
    pos: Position,
    sprite: uint,
}

impl Player {
    pub fn new(pos: Vector2i, sprite: uint) -> Player {
        Player {
            pos: Position{ p: pos },
            sprite: sprite,
        }
    }

    pub fn tile(&self) -> Tile {
        Tile{
            value: self.sprite,
            position: self.pos.p
        }
    }
}
