use rsfml::system::vector2::Vector2i;

// A tile is a single cell in the dungeon
pub struct Tile {
    value: u8,
    position: Vector2i
}

impl Tile {
    pub fn new(pos: Vector2i, value: u8) -> Tile {
        Tile{
            value: value,
            position: pos
        }
    }
}

// Floor represent a level of the dungeon
pub struct Floor {
    dimension: Vector2i,
    tiles: ~[Tile]
}

impl Floor {
    pub fn new(dim: Vector2i) -> Floor {
        let mut tiles: ~[Tile] = ~[];
        for x in range(0, dim.x) {
            for y in range(0, dim.y) {
                tiles.push(Tile::new(Vector2i::new(x, y), 0));
            }
        }
        Floor{
            dimension: dim,
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

    // pub fn tile_at<'r>(&'r self, pos: Vector2i) -> &'r Tile {
    //     return &self.tiles[self.dimension.x * pos.y + pos.x]
    // }

}

// A dungeon is composed of multiple floors
pub struct Dungeon {
    dimension: Vector2i,
    floor: uint,
    floors: ~[Floor]
}

impl Dungeon {
    pub fn new(dim: Vector2i, depth: uint) -> ~Dungeon {
        ~Dungeon{
            floor: depth - 1,
            dimension: dim,
            floors: Floor::new_multiple(dim, depth),
        }
    }

    pub fn current_floor<'r>(&'r self) -> &'r Floor {
        return &self.floors[self.floor]
    }
}
