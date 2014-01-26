use rsfml::graphics::{RenderWindow};
use rsfml::window::{event, keyboard};

// Processes all available window/keyboard events
pub fn handle(window: &mut RenderWindow) {
    loop {
        if handle_window_event(window) {
            break
        }
    }
}

// Returns true if NoEvent was encountered
fn handle_window_event(window: &mut RenderWindow) -> bool {
    let mut consumed = false;
    let e = window.poll_event();
    match e {
        event::Closed => window.close(),
        event::KeyReleased{..} => handle_key_released(window, e),
        event::NoEvent => consumed = true,
        _ => {}
    };
    consumed
}

// Triggers callbacks for key release events
fn handle_key_released(window: &mut RenderWindow, e: event::Event) {
    let code = match e {
        event::KeyReleased{code: code, ..} => code,
        _ => fail!("Event is not a KeyReleased event")
    };
    match code {
        keyboard::Escape => window.close(),
        _ => {}
    };
}
