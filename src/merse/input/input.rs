use rsfml::graphics::{RenderWindow};
use rsfml::window::{event, keyboard};
use units::Player;
use dungeon::{Dungeon};

/// Processes all available window/keyboard events
pub fn handle(window: &mut RenderWindow, dungeon: &mut Dungeon,
              player: &mut Player) {
    loop {
        let e = window.poll_event();
        match e {
            event::NoEvent => break,
            _ => {}
        }
        handle_window_event(window, e);
        handle_player_event(player, dungeon, e);
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

fn handle_player_event(player: &mut Player, dungeon: &Dungeon,
                       e: event::Event) {
    match e {
        event::KeyReleased{code: code, ..} => match code {
            keyboard::Left => player.pos.move(dungeon, -1, 0),
            keyboard::Right => player.pos.move(dungeon, 1, 0),
            keyboard::Up => player.pos.move(dungeon, 0, -1),
            keyboard::Down => player.pos.move(dungeon, 0, 1),
            keyboard::Comma => player.pos.descend(dungeon),
            keyboard::Period => player.pos.ascend(dungeon),
            _ => {}
        },
        _ => {}
    }
}
