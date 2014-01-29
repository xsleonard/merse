use rsfml::system::vector2::Vector2i;
use self::position::Position;
use config::SpriteConfig;

mod position;

pub struct Player {
    pos: Position,
    sprite: SpriteConfig,
}

impl Player {
    pub fn new(pos: Vector2i, sprite: SpriteConfig) -> Player {
        Player {
            pos: Position{ p: pos },
            sprite: sprite,
        }
    }
}
