extern crate game_2048_model;
extern crate game_2048_view;
extern crate sdl2;

use game_2048_model::models::{ArrayModel, Directions, Model};
use game_2048_view::{Square, View};
use rand::thread_rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;
use std::time::Duration;

type State = Vec<Square>;

enum Direction {
    Base(Directions),
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
                } => Direction::Base(Directions::Up),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => Direction::Base(Directions::Down),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => Direction::Base(Directions::Left),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => Direction::Base(Directions::Right),
                _ => Direction::NONE,
            }
        }
        return direction;
    }
}

fn array_model_to_state(game: &ArrayModel) -> State {
    let mut state: State = vec![];
    let mut x = 0;
    let mut y = 0;
    let array = game.as_array();
    for value in array.iter() {
        if *value != 0 {
            state.push(Square {
                x,
                y,
                value: *value as u32,
            });
        }
        x += 1;
        if x >= 4 {
            x = 0;
            y += 1;
        }
    }
    state
}

pub fn main() {
    let mut game = ArrayModel::new();
    let mut rng = thread_rng();
    game.random(&mut rng).unwrap();
    game.random(&mut rng).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let mut view = View::new(
        &sdl_context,
        Color::RGB(127, 127, 127),
        Color::RGB(0, 0, 0),
        4,
        800,
    );
    view.draw();
    view.update(array_model_to_state(&game));
    let mut player: HumanPlayer = sdl_context.event_pump().unwrap();
    loop {
        match player.get() {
            Direction::Base(dir) => {
                if game.slide(dir).is_some() {
                    match game.random(&mut rng) {
                        Err(err) => {
                            println!("Game Over: {}", err);
                            break;
                        }
                        Ok(_) => (),
                    };
                    view.update(array_model_to_state(&game));
                }
            }
            Direction::ESCAPE => break,
            Direction::NONE => {}
        };
        view.draw();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
