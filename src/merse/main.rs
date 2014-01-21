#[desc = "Exploratory Rust roguelike."];
#[license = "GPLv2"];

extern mod native;
extern mod rsfml;
// extern mod sdl2;
// extern mod sdl2_image;

mod gui;
mod input;

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
    let mut window = gui::init(~"Merse");
    while window.is_open() {
        input::handle(window);
        gui::display(window);
    }
}