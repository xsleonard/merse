#[desc = "Exploratory Rust roguelike."];
#[license = "GPLv2"];

extern mod native;
extern mod rsfml;
extern mod extra;
// extern mod sdl2;
// extern mod sdl2_image;

use dungeon::Dungeon;
use units::Player;
use rsfml::system::vector2::Vector2i;

mod gui;
mod input;
mod config;
mod dungeon;
mod units;

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

    let mut dungeon = ~Dungeon::new(c.dungeon.size(), c.dungeon.depth);
    let spriteset = c.get_spritesets();
    let main_spriteset = spriteset.get(&~"main");
    let mut player = ~Player::new(dungeon.center(),
                                  main_spriteset.get(&~"player").clone());
    dungeon.generate_terrain(main_spriteset);

    let mut gui_state = gui::Gui::new(&c.window, c.spritesheets);
    while gui_state.window.is_open() {
        input::handle(gui_state.window, dungeon, player);
        let floor = dungeon.current_floor();
        gui_state.display(floor, player);
    }
}
