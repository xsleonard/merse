use extra::json;
use extra::serialize::Decodable;
use std::path::Path;
use std::io::fs::File;
use rsfml::system::vector2::Vector2i;
use std::hashmap::HashMap;

#[deriving(Decodable)]
pub struct Config {
    window: WindowConfig,
    spritesheets: ~[SpritesheetConfig],
    spritesets: ~[SpritesetConfig],
    dungeon: DungeonConfig
}

impl Config {
    // Returns a map from spritesheet names to their configs
    pub fn get_spritesheets(&self) -> ~HashMap<~str, SpritesheetConfig> {
        let mut h: ~HashMap<~str, SpritesheetConfig> = ~HashMap::new();
        for s in self.spritesheets.iter() {
            h.insert((*s).name.clone(), s.clone());
        }
        h
    }

    // Returns a map from spritesheet name to a spriteset
    pub fn get_spritesets(&self) -> ~HashMap<~str, Spriteset> {
        let mut h: ~HashMap<~str, Spriteset> = ~HashMap::new();
        let sheets = self.get_spritesheets();
        for s in self.spritesets.iter() {
            let set = s.clone();
            let sheet = sheets.get(&set.name);
            let mut ih: Spriteset = ~HashMap::new();
            for t in set.tiles.iter() {
                let tile = t.clone();
                let val = sheet.tiles_wide * tile.y + tile.x;
                ih.insert(tile.name, val);
            }
            h.insert(set.name, ih);
        }
        h
    }
}

#[deriving(Decodable)]
pub struct DungeonConfig {
    height: uint,
    width: uint,
    depth: uint
}

impl DungeonConfig {
    pub fn size(&self) -> Vector2i {
        Vector2i::new(self.width as i32, self.height as i32)
    }
}

#[deriving(Decodable)]
#[deriving(Clone)]
pub struct SpritesheetConfig {
    name: ~str,
    filename: ~str,
    tile_size: uint,
    tiles_wide: uint
}

#[deriving(Decodable)]
#[deriving(Clone)]
pub struct SpritesetConfig {
    name: ~str,
    tiles: ~[~TileConfig]
}

// Maps from tile names to indices in the spritesheet
pub type Spriteset = ~HashMap<~str, uint>;

#[deriving(Decodable)]
#[deriving(Clone)]
pub struct TileConfig {
    name: ~str,
    x: uint,
    y: uint
}

#[deriving(Decodable)]
pub struct WindowConfig {
    name: ~str,
    width: uint,
    height: uint,
    fullscreen: bool
}

impl WindowConfig {
    pub fn size(&self) -> Vector2i {
        Vector2i::new(self.width as i32, self.height as i32)
    }
}

// Loads a json config file from filename into a Config structure
pub fn load_config(filename: ~str) -> ~Config {
    // read config from file
    let data = read_config(filename);
    let jsonobj = json::from_str(data);
    let mut decoder = json::Decoder::new(jsonobj.unwrap());
    ~Decodable::decode(&mut decoder)
}

fn read_config(filename: ~str) -> ~str {
    let path = Path::new(filename);
    let on_error = || fail!("open of {:?} failed", path);
    File::open(&path).unwrap_or_else(on_error).read_to_str()
}

