#[desc = "Exploratory Rust roguelike."];
#[license = "GPLv2"];

extern mod native;
extern mod rsfml;
extern mod extra;
// extern mod sdl2;
// extern mod sdl2_image;

use rsfml::system::vector2::Vector2i;
use dungeon::Dungeon;

mod gui;
mod input;
mod config;
mod dungeon;

// Override the runtime start fn to guarantee the window is run on the main
// thread
// TODO -- understand exactly what this is doing
#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

// TODO -- again, what is this doing exactly
#[main]
fn main() {
    let c = config::load_config(~"./settings.json");

    let dungeon_size = Vector2i::new(c.dungeon.width as i32,
                                     c.dungeon.height as i32);
    let dungeon = Dungeon::new(dungeon_size, c.dungeon.depth);

    let window_size = Vector2i::new(c.width as i32, c.height as i32);
    let mut gui_state = gui::Gui::new(c.app_name.clone(), c.spritesheets,
                                      window_size, c.fullscreen);
    while gui_state.window.is_open() {
        input::handle(gui_state.window);
        gui_state.display(dungeon.current_floor());
    }
}
