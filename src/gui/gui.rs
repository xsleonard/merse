use sdl2;
use sdl2::surface::Surface;
use sdl2_image;
use sdl2_image::{LoadSurface, LoadTexture};


pub fn display(app_name: &str) {
    sdl2::init([sdl2::InitVideo]);
    sdl2_image::init([sdl2_image::InitPng]);

    let window = match sdl2::video::Window::new(
        app_name, sdl2::video::PosCentered, sdl2::video::PosCentered,
        800, 600, [sdl2::video::OpenGL]) {
        Ok(window) => window,
        Err(err) => fail!(format!("Failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(
        window, sdl2::render::DriverAuto, [sdl2::render::Accelerated]) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("Failed to create window: {}", err))
    };

    renderer.set_draw_color(sdl2::pixels::RGB(66,122,66));
    renderer.clear();
    renderer.present();

    // Load a surface, and convert it to a texture bound to the renderer
    let surface: ~Surface = match LoadSurface::from_file("test.png") {
        Ok(surface) => surface,
        Err(err) => fail!(format!("Failed to load png: {}", err))
    };
    let texture = match renderer.create_texture_from_surface(surface) {
        Ok(texture) => texture,
        Err(err) => fail!(format!("Failed to create surface: {}", err))
    };

    // Load a texture directly via the renderer
    // let texture = match renderer.load_texture("test.png") {
    //     Ok(texture) => texture,
    //     Err(err) => fail!(format!("Could not set render target: {}", err))
    // };

    renderer.copy(texture, None, None);
    renderer.present();

    'main: loop {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::QuitEvent(_) => break 'main,
                sdl2::event::KeyDownEvent(_, _, key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        break 'main
                    }
                }
                _ => {}
            }
        }
    }

    sdl2_image::quit();
    sdl2::quit();
}

// use sdl2;

// pub fn main() {
//     sdl2::init([sdl2::InitVideo]);

//     let window = match sdl2::video::Window::new("rust-sdl2 demo: Video", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, [sdl2::video::OpenGL]) {
//         Ok(window) => window,
//         Err(err) => fail!(format!("failed to create window: {}", err))
//     };

//     let renderer = match sdl2::render::Renderer::from_window(window,
//         sdl2::render::DriverAuto, [sdl2::render::Accelerated]) {
//         Ok(renderer) => renderer,
//         Err(err) => fail!(format!("failed to create renderer: {}", err))
//     };

//     renderer.set_draw_color(sdl2::pixels::RGB(255, 0, 0));
//     renderer.clear();
//     renderer.present();

//     'main : loop {
//         'event : loop {
//             match sdl2::event::poll_event() {
//                 sdl2::event::QuitEvent(_) => break 'main,
//                 sdl2::event::KeyDownEvent(_, _, key, _, _) => {
//                     if key == sdl2::keycode::EscapeKey {
//                         break 'main
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }

//     sdl2::quit();
// }
