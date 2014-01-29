use std::hashmap::HashMap;
use rsfml::window::{ContextSettings, VideoMode, Close, Fullscreen};
use rsfml::graphics::{Texture, Color, RenderWindow, Sprite};
use rsfml::graphics::rect::IntRect;
use rsfml::traits::drawable::Drawable;
use rsfml::system::vector2::{Vector2i, Vector2f, ToVec};
use dungeon::{Dungeon};
use units::Player;
use config::{SpriteConfig, WindowConfig, SpritesheetConfigs};

struct Spritesheet {
    texture: Texture,
    tile_size: uint,
    tiles_wide: uint
}

type Spritesheets = HashMap<~str, Spritesheet>;

impl Spritesheet {
    fn get_pos(&self, value: uint) -> Vector2i {
        return Vector2i::new((self.tile_size * (value % self.tiles_wide)) as i32,
                             (self.tile_size * (value / self.tiles_wide)) as i32);
    }

    fn get_rect(&self, value: uint) -> IntRect {
        let pos = self.get_pos(value);
        let ts = self.tile_size as i32;
        IntRect::new(pos.x, pos.y, ts, ts)
    }

    fn place_sprite(&self, s: &mut Sprite, c: &SpriteConfig, p: Vector2i) {
        // set the subtexture area
        let rect = self.get_rect(c.val);
        s.set_texture_rect(&rect);
        // set the screen position
        let pos = self.get_screen_pos(p);
        s.set_position(&pos);
    }

    fn get_screen_pos(&self, pos: Vector2i) -> Vector2f {
        Vector2i::new(pos.x * (self.tile_size as i32),
                      pos.y * (self.tile_size as i32)).to_vector2f()
    }
}

pub struct Gui {
    size: Vector2i,
    window: ~RenderWindow,
    spritesheets: ~Spritesheets
}

impl Gui {
    pub fn new(wcfg: &WindowConfig,
               spritesheets: &SpritesheetConfigs) -> ~Gui {
        let window = load_window(wcfg.name.clone(), wcfg.size(),
                                 wcfg.fullscreen);
        ~Gui {
            size: wcfg.size(),
            window: window,
            spritesheets: ~load_spritesheets(spritesheets),
        }
    }

    pub fn display(&mut self, dungeon: &Dungeon, player: &Player) {
        // Clear the window
        self.window.clear(&Color::new_RGB(0, 200, 200));
        self.draw_sprites(dungeon);
        self.draw_player(player);
        // Flip the screen
        self.window.display();
    }

    fn draw_sprites(&mut self, dungeon: &Dungeon) {
        // Iterate the dungeon floor,
        // lookup its sprite to draw,
        // set that sprites position to current pos
        // window.draw_sprite(sprite)
        let ss = self.spritesheets.get(&~"main");
        let mut s = ~Sprite::new_with_texture(&ss.texture).expect("No sprite");
        let floor = dungeon.current_floor();
        for t in floor.tiles.iter() {
            let c = dungeon.sprite(t.sprite);
            ss.place_sprite(s, c, t.position);
            s.draw_in_render_window(self.window);
        }
    }

    fn draw_player(&mut self, player: &Player) {
        let ss = self.spritesheets.get(&~"main");
        let mut s = ~Sprite::new_with_texture(&ss.texture).expect("No sprite");
        ss.place_sprite(s, &player.sprite, player.pos.p);
        s.draw_in_render_window(self.window);
    }
}

fn load_window(name: ~str, size: Vector2i,
               fullscreen: bool) -> ~RenderWindow {
    let settings = ContextSettings::default();
    let video = VideoMode::new_init(size.x as uint, size.y as uint, 32);
    let mode = if fullscreen {
        Fullscreen
    } else {
        Close
    };
    match RenderWindow::new(video, name, mode, &settings) {
        Some(window) => ~window,
        None => fail!("Couldn't create a new Render Window.")
    }
}

fn load_spritesheets(config: &SpritesheetConfigs) -> Spritesheets {
    let mut spritesheets: Spritesheets = HashMap::new();
    for (_, cfg) in config.iter() {
        let filename = cfg.filename.clone();
        let tx = match Texture::new_from_file(filename) {
            Some(tex) => tex,
            None =>  fail!(print!("Failed to load {} to texture", filename))
        };
        spritesheets.insert(cfg.name.clone(), Spritesheet{
            texture: tx,
            tile_size: cfg.tile_size,
            tiles_wide: cfg.tiles_wide
        });
    }
    spritesheets
}
