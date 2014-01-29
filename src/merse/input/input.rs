use rsfml::graphics::{RenderWindow};
use rsfml::window::{event, keyboard};
use units::Player;
use dungeon::{Dungeon, Floor};

/// Processes all available window/keyboard events
pub fn handle(window: &mut RenderWindow, dungeon: &mut Dungeon,
              player: &mut Player) {
    let floor = dungeon.current_floor();
    loop {
        let e = window.poll_event();
        match e {
            event::NoEvent => break,
            _ => {}
        }
        handle_window_event(window, e);
        handle_player_event(player, floor, e);
    }
}

fn handle_window_event(window: &mut RenderWindow, e: event::Event) {
    match e {
        event::Closed => window.close(),
        event::KeyReleased{code: code, ..} => match code {
            keyboard::Escape => window.close(),
            _ => {}
        },
        _ => {}
    };
}

fn handle_player_event(player: &mut Player, floor: &Floor, e: event::Event) {
    match e {
        event::KeyReleased{code: code, ..} => match code {
            keyboard::Left => player.pos.move(floor, -1, 0),
            keyboard::Right => player.pos.move(floor, 1, 0),
            keyboard::Up => player.pos.move(floor, 0, -1),
            keyboard::Down => player.pos.move(floor, 0, 1),
            _ => {}
        },
        _ => {}
    }
}
