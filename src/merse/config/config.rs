use std::path::Path;
use std::io::fs::File;
use std::hashmap::HashMap;
use extra::json;
use extra::serialize::Decodable;
use rsfml::system::vector2::Vector2i;

/// More advanced structures need to be built from the json data, so we
/// convert a DecodableConfig to this
pub struct Config {
    window: WindowConfig,
    spritesheets: SpritesheetConfigs,
    sprites: SpriteConfigs,
    dungeon: DungeonConfig
}

impl Config {
    pub fn from_decodable_config(c: &DecodableConfig) -> Config {
        Config{
            window: c.window.clone(),
            dungeon: c.dungeon.clone(),
            spritesheets: c.get_spritesheets(),
            sprites: c.get_sprites(),
        }
    }
}

/// This contains the JSON representation
#[deriving(Decodable)]
pub struct DecodableConfig {
    window: WindowConfig,
    spritesheets: ~[SpritesheetConfig],
    spritesets: ~[SpritesetConfig],
    dungeon: DungeonConfig
}

impl DecodableConfig {
    // Returns a map from spritesheet names to their configs
    pub fn get_spritesheets(&self) -> SpritesheetConfigs {
        let mut h: SpritesheetConfigs = HashMap::new();
        for s in self.spritesheets.iter() {
            h.insert((*s).name.clone(), s.clone());
        }
        h
    }

    // Returns a map from sprite names to their configs
    pub fn get_sprites(&self) -> SpriteConfigs {
        let mut h: SpriteConfigs = HashMap::new();
        let sheets = self.get_spritesheets();

        for s in self.spritesets.iter() {
            let set_name = (*s).name.clone();
            let sheet = sheets.get(&set_name);
            for t in (*s).tiles.iter() {
                let tile = t.clone();
                if h.contains_key_equiv(&tile.name) {
                    fail!(format!("Duplicate tile \"{}\"", tile.name))
                }
                let val = sheet.tiles_wide * tile.y + tile.x;
                h.insert(tile.name, TileConfig{
                    val: val,
                    spritesheet: set_name.clone(),
                    name: t.name.clone()
                });
            }
        }
        h
    }
}

#[deriving(Decodable)]
#[deriving(Clone)]
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
    tiles: ~[~DecodableTileConfig]
}

#[deriving(Decodable)]
#[deriving(Clone)]
pub struct DecodableTileConfig {
    name: ~str,
    x: uint,
    y: uint
}

#[deriving(Clone)]
pub struct TileConfig {
    spritesheet: ~str,
    name: ~str,
    val: uint,
}

// Maps from tile names to TileConfigs
pub type SpriteConfigs = HashMap<~str, TileConfig>;

// Maps from spritesheet name to SpritesheetConfigs
pub type SpritesheetConfigs = HashMap<~str, SpritesheetConfig>;

#[deriving(Decodable)]
#[deriving(Clone)]
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
    let config: ~DecodableConfig = ~Decodable::decode(&mut decoder);
    ~Config::from_decodable_config(config)
}

fn read_config(filename: ~str) -> ~str {
    let path = Path::new(filename);
    let on_error = || fail!("open of {:?} failed", path);
    File::open(&path).unwrap_or_else(on_error).read_to_str()
}

