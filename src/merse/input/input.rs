use rsfml::graphics::{RenderWindow};
use rsfml::window::{event, keyboard};

pub fn handle(window: &mut RenderWindow) {
    loop {
        if (handle_window_event(window)) {
            break
        }
    }
}

// Returns true if NoEvent was encountered
fn handle_window_event(window: &mut RenderWindow) -> bool {
    let mut consumed = false;
    match window.poll_event() {
        event::Closed => window.close(),
        event::KeyReleased{code: code, alt: _, ctrl: _, shift: _,
                           system: _} => match code {
            keyboard::Escape => window.close(),
            _ => {}
        },
        event::NoEvent => consumed = true,
        _ => {}
    };
    consumed
}
