use rsfml::window::{ContextSettings, VideoMode, Close, Fullscreen};
use rsfml::graphics::{Texture, Color, RenderWindow, Sprite};
use rsfml::graphics::rect::IntRect;
use rsfml::traits::drawable::Drawable;
use rsfml::system::vector2::{Vector2i, Vector2f, ToVec};
use dungeon::Floor;
use config::{SpritesheetConfig, WindowConfig};

struct Spritesheet {
    texture: Texture,
    tile_size: uint,
    tiles_wide: uint
}

impl Spritesheet {
    fn get_pos(&self, value: uint) -> Vector2i {
        return Vector2i::new((self.tile_size * (value % self.tiles_wide)) as i32,
                             (self.tile_size * (value / self.tiles_wide)) as i32);
    }
}

pub struct Gui {
    size: Vector2i,
    window: ~RenderWindow,
    spritesheets: ~[Spritesheet]
}

impl Gui {
    pub fn new(wcfg: &WindowConfig, spritesheets: &[SpritesheetConfig]) -> ~Gui {
        let window = load_window(wcfg.name.clone(), wcfg.size(),
                                 wcfg.fullscreen);
        ~Gui {
            size: wcfg.size(),
            window: window,
            spritesheets: load_spritesheets(spritesheets),
        }
    }

    pub fn display(&mut self, floor: &Floor) {
        // Clear the window
        self.window.clear(&Color::new_RGB(0, 200, 200));
        self.draw_sprites(floor);
        // Display things on screen
        self.window.display();
    }

    fn draw_sprites(&mut self, floor: &Floor) {
        // Iterate the dungeon floor,
        // lookup its sprite to draw,
        // set that sprites position to current pos
        // window.draw_sprite(sprite)
        let ss = &self.spritesheets[0];
        let mut s = Sprite::new_with_texture(&ss.texture).expect("No sprite");

        for t in floor.tiles.iter() {
            let pos = self.get_screen_pos(t.position, ss.tile_size);
            let texpos = self.spritesheets[0].get_pos(t.value as uint);
            // set the subtexture area
            let rect = IntRect::new(texpos.x, texpos.y,
                                    ss.tile_size as i32, ss.tile_size as i32);
            s.set_texture_rect(&rect);
            // set the screen position
            s.set_position(&pos);
            s.draw_in_render_window(self.window)
        }
    }

    fn get_screen_pos(&self, pos: Vector2i,
                      tilesize: uint) -> Vector2f {
        Vector2i::new(pos.x * (tilesize as i32),
                      pos.y * (tilesize as i32)).to_vector2f()
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

fn load_spritesheets(config: &[SpritesheetConfig]) -> ~[Spritesheet] {
    let mut spritesheets: ~[Spritesheet] = ~[];
    for c in config.iter() {
        let cfg = c.clone();
        let filename = cfg.filename.clone();
        let tx = match Texture::new_from_file(filename) {
            Some(tex) => tex,
            None =>  fail!(print!("Failed to load {} to texture", filename))
        };
        spritesheets.push(Spritesheet{
            texture: tx,
            tile_size: cfg.tile_size,
            tiles_wide: cfg.tiles_wide
        });
    }
    spritesheets
}
