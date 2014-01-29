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
            h.insert(s.name.clone(), s.clone());
        }
        h
    }

    // Returns a map from sprite names to their configs
    pub fn get_sprites(&self) -> SpriteConfigs {
        let mut h: SpriteConfigs = SpriteConfigs::new();
        let sheets = self.get_spritesheets();
        for s in self.spritesets.iter() {
            let sheet = sheets.get(&s.name);
            for t in s.tiles.iter() {
                h.add(t.clone(), sheet);
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
    tiles: ~[~DecodableSpriteConfig]
}

#[deriving(Decodable)]
#[deriving(Clone)]
pub struct DecodableSpriteConfig {
    name: ~str,
    x: uint,
    y: uint,
    solid: bool,
}

#[deriving(Clone)]
pub struct SpriteConfig {
    index: uint,
    spritesheet: ~str,
    name: ~str,
    val: uint,
    solid: bool,
}

impl SpriteConfig {
    pub fn from_decodable_config(tile: &DecodableSpriteConfig,
                                 sheet: &SpritesheetConfig,
                                 index: uint) -> SpriteConfig {
        SpriteConfig{
            index: index,
            spritesheet: sheet.name.clone(),
            name: tile.name.clone(),
            solid: tile.solid.clone(),
            val: sheet.tiles_wide * tile.y + tile.x,
        }
    }
}

// Maps from tile names to SpriteConfigs
#[deriving(Clone)]
pub struct SpriteConfigs {
    map: HashMap<~str, SpriteConfig>,
    arr: ~[SpriteConfig]
}

impl SpriteConfigs {
    pub fn new() -> SpriteConfigs {
        SpriteConfigs{
            map: HashMap::new(),
            arr: ~[],
        }
    }

    pub fn add(&mut self, tile: &DecodableSpriteConfig,
               sheet: &SpritesheetConfig) -> SpriteConfig {
        if self.map.contains_key_equiv(&tile.name) {
            fail!(format!("Duplicate tile \"{}\"", tile.name))
        }
        let sc = SpriteConfig::from_decodable_config(tile, sheet,
                                                     self.arr.len());
        self.map.insert(tile.name.clone(), sc.clone());
        self.arr.push(sc.clone());
        sc
    }
}

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

