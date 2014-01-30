use self::position::Position;
use config::SpriteConfig;
use dungeon::Dungeon;

mod position;

pub struct Player {
    pos: Position,
    sprite: SpriteConfig,
}

impl Player {
    pub fn new(sprite: SpriteConfig, dungeon: &Dungeon) -> Player {
        Player {
            pos: Position{
                p: dungeon.center(),
                floor: dungeon.floors.len() - 1,
            },
            sprite: sprite,
        }
    }
}
