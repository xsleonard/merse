use rsfml::window::{ContextSettings, VideoMode, Close, Fullscreen};
use rsfml::graphics::{Color, RenderWindow};

pub fn init(name: ~str) -> ~RenderWindow {
    let setting = ContextSettings::default();
    let video = VideoMode::new_init(800, 600, 32);
    let window = match RenderWindow::new(video, name, Close, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };
    ~window
}

pub fn display(window: &mut RenderWindow) {
    // Clear the window
    window.clear(&Color::new_RGB(0, 200, 200));
    // Display things on screen
    window.display()
}
