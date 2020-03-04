extern crate game_2048_view;
extern crate sdl2;

use game_2048_view::{Square, View};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;
use std::time::Duration;

type State = Vec<Square>;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ESCAPE,
    NONE,
}

trait Player {
    fn get(&mut self) -> Direction;
}

type HumanPlayer = EventPump;

impl Player for HumanPlayer {
    fn get(&mut self) -> Direction {
        let mut direction = Direction::NONE;
        for event in self.poll_iter() {
            direction = match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => Direction::ESCAPE,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => Direction::UP,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => Direction::DOWN,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => Direction::LEFT,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => Direction::RIGHT,
                _ => Direction::NONE,
            }
        }
        return direction;
    }
}

fn up() -> State {
    return vec![Square {
        x: 0,
        y: 0,
        value: 2,
    }];
}

fn right() -> State {
    return vec![Square {
        x: 3,
        y: 0,
        value: 4,
    }];
}

fn down() -> State {
    return vec![Square {
        x: 3,
        y: 3,
        value: 8,
    }];
}

fn left() -> State {
    return vec![
        Square {
            x: 0,
            y: 3,
            value: 16,
        },
        Square {
            x: 1,
            y: 3,
            value: 2,
        },
    ];
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut view = View::new(
        &sdl_context,
        Color::RGB(127, 127, 127),
        Color::RGB(0, 0, 0),
        4,
        800,
    );
    view.draw();
    let mut player: HumanPlayer = sdl_context.event_pump().unwrap();
    loop {
        match player.get() {
            Direction::UP => view.update(up()),
            Direction::DOWN => view.update(down()),
            Direction::LEFT => view.update(left()),
            Direction::RIGHT => view.update(right()),
            Direction::ESCAPE => break,
            Direction::NONE => {}
        };

        view.draw();

        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
