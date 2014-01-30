use std::rand::{random, Rand};
use std::ops::{Sub, Add, Rem};
use std::vec::build;
use rsfml::system::vector2::Vector2i;
use config::{SpriteConfigs, SpriteConfig};
use units::Player;

// A tile is a single cell in the dungeon
pub struct Tile {
    sprite: uint,
    position: Vector2i
}

impl Tile {
    pub fn new(pos: Vector2i, sprite: uint) -> Tile {
        Tile{
            sprite: sprite,
            position: pos
        }
    }
}

// Floor represent a level of the dungeon
pub struct Floor {
    dim: Vector2i,
    level: uint,
    tiles: ~[Tile],
    downstairs: Vector2i,
}

impl Floor {
    pub fn new(dim: Vector2i, level: uint) -> Floor {
        let mut tiles: ~[Tile] = ~[];
        for y in range(0, dim.y) {
            for x in range(0, dim.x) {
                tiles.push(Tile::new(Vector2i::new(x, y), 0));
            }
        }
        Floor{
            dim: dim,
            tiles: tiles,
            level: level,
            downstairs: Vector2i::new(0, 0),
        }
    }

    pub fn new_multiple(dim: Vector2i, depth: uint) -> ~[Floor] {
        let mut floors: ~[Floor] = ~[];
        for level in range(0, depth) {
            floors.push(Floor::new(dim, level));
        }
        floors
    }

    fn index(&self, x: i32, y: i32) -> i32 {
        y * self.dim.x + x
    }

    fn vindex(&self, v: Vector2i) -> i32 {
        v.y * self.dim.x + v.x
    }

    fn uindex(&self, x: uint, y: uint) -> uint {
        y * (self.dim.x as uint) + x
    }

    fn generate_terrain(&mut self, scfg: &SpriteConfigs,
                        upstairs: Option<Vector2i>) {
        let sprites = &scfg.map;
        for x in range(0, self.dim.x) {
            for y in range(0, self.dim.y) {
                let i = self.index(x, y);
                self.tiles[i].sprite = sprites.get(&~"floor").index;
            }
        }
        for x in range(0, self.dim.x) {
            let top = self.index(x, 0);
            self.tiles[top].sprite = sprites.get(&~"wall").index;
            let bottom = self.index(x, self.dim.y-1);
            self.tiles[bottom].sprite = sprites.get(&~"wall").index;
        }
        for y in range(0, self.dim.y) {
            let left = self.index(0, y);
            self.tiles[left].sprite = sprites.get(&~"wall").index;
            let right = self.index(self.dim.x-1, y);
            self.tiles[right].sprite = sprites.get(&~"wall").index;
        }

        if self.level > 0 {
            self.downstairs = Vector2i::new(
                randrange(1, self.dim.x as uint - 1) as i32,
                randrange(1, self.dim.y as uint - 1) as i32
            );
            let down = self.vindex(self.downstairs);
            let s = sprites.get(&~"stairs_down");
            self.tiles[down].sprite = s.index;
        }

        match upstairs {
            Some(v) => {
                let up = self.vindex(v);
                let s = sprites.get(&~"stairs_up");
                self.tiles[up].sprite = s.index;
            },
            _ => {}
        }
    }

    pub fn tile_at<'r>(&'r self, pos: Vector2i) -> &'r Tile {
        return &self.tiles[self.dim.x * pos.y + pos.x]
    }
}

// A dungeon is composed of multiple floors
pub struct Dungeon {
    dim: Vector2i,
    floor: uint,
    floors: ~[Floor],
    sprites: ~SpriteConfigs,
}

impl Dungeon {
    pub fn new(sprites: &SpriteConfigs, dim: Vector2i, depth: uint) -> Dungeon {
        Dungeon{
            floor: depth - 1,
            dim: dim,
            floors: Floor::new_multiple(dim, depth),
            sprites: ~sprites.clone(),
        }
    }

    /// Returns the center coordinate of the current floor
    pub fn center(&self) -> Vector2i {
        Vector2i::new(self.dim.x / 2, self.dim.y / 2)
    }

    pub fn current_floor<'r>(&'r self) -> &'r Floor {
        &self.floors[self.floor]
    }

    pub fn floor_above<'r>(&'r self) -> Option<&'r Floor> {
        if self.floor == self.floors.len() - 1 {
            None
        } else {
            Some(&self.floors[self.floor + 1])
        }
    }

    // Create the terrain for all the dungeon floor
    pub fn generate_terrain(&mut self) {
        let depth = self.floors.len();
        for f in self.floors.mut_rev_iter() {
            let above = if f.level == depth - 1 {
                None
            } else {
                Some(self.floors[f.level + 1].downstairs)
            };
            f.generate_terrain(self.sprites, above);
        }
    }

    pub fn sprite<'r>(&'r self, sprite: uint) -> &'r SpriteConfig {
        &self.sprites.arr[sprite]
    }

    pub fn update(&mut self, player: &Player) {
        self.floor = player.pos.floor;
    }
}


fn randrange<T: Rand + Sub<T, T> + Add<T, T> + Rem<T, T>>(a: T, b: T) -> T {
    let r: T = random();
    a + (r % (b - a))
}
