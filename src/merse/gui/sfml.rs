use rsfml::window::{ContextSettings, VideoMode, Close, Fullscreen};
use rsfml::graphics::{Texture, Color, RenderWindow, Sprite};
use rsfml::traits::drawable::Drawable;
use rsfml::system::vector2::Vector2f;

pub struct Gui {
    window: ~RenderWindow,
    textures: ~[Texture]
}

impl Gui {
    pub fn new(name: ~str, imagefiles: &[~str], width: uint, height: uint,
               fullscreen: bool) -> ~Gui {
        let window = load_window(name, width, height, fullscreen);
        ~Gui {
            window: window,
            textures: load_textures(imagefiles),
        }
    }

    pub fn display(&mut self) {
        // Clear the window
        self.window.clear(&Color::new_RGB(0, 200, 200));
        self.draw_sprites();
        // Display things on screen
        self.window.display();
    }

    pub fn draw_sprites(&mut self) {
        // Iterate the dungeon floor,
        // lookup its sprite to draw,
        // set that sprites position to current pos
        // window.draw_sprite(sprite)
        for tx in self.textures.iter() {
            let mut sprite = Sprite::new_with_texture(tx).expect("No sprite");
            sprite.set_position(&Vector2f::new(0., 0.));
            sprite.draw_in_render_window(self.window)
        }
    }

}

fn load_window(name: ~str, width: uint, height: uint,
               fullscreen: bool) -> ~RenderWindow {
    let settings = ContextSettings::default();
    let video = VideoMode::new_init(width, height, 32);
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

fn load_textures(filenames: &[~str]) -> ~[Texture] {
    let mut textures: ~[Texture] = ~[];
    for filename in filenames.iter() {
        match Texture::new_from_file(filename.clone()) {
            Some(tex) => textures.push(tex),
            None =>  fail!(print!(
                "Failed to load {} to texture", filename.clone()))
        }
    }
    textures
}
