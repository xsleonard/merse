use std::hashmap::HashMap;
use rsfml::system::vector2::Vector2i;
use config::{SpriteConfigs, SpriteConfig};

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
    tiles: ~[Tile]
}

impl Floor {
    pub fn new(dim: Vector2i) -> Floor {
        let mut tiles: ~[Tile] = ~[];
        for y in range(0, dim.y) {
            for x in range(0, dim.x) {
                tiles.push(Tile::new(Vector2i::new(x, y), 0));
            }
        }
        Floor{
            dim: dim,
            tiles: tiles,
        }
    }

    pub fn new_multiple(dim: Vector2i, depth: uint) -> ~[Floor] {
        let mut floors: ~[Floor] = ~[];
        for _ in range(0, depth) {
            floors.push(Floor::new(dim));
        }
        floors
    }

    fn index(&self, x: i32, y: i32) -> i32 {
        y * self.dim.x + x
    }

    fn generate_terrain(&mut self, sprites: &HashMap<~str, SpriteConfig>) {
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

    // Create the terrain for all the dungeon floor
    pub fn generate_terrain(&mut self, sprites: &SpriteConfigs) {
        for f in self.floors.mut_iter() {
            f.generate_terrain(&sprites.map);
        }
    }

    pub fn sprite<'r>(&'r self, sprite: uint) -> &'r SpriteConfig {
        &self.sprites.arr[sprite]
    }
}
