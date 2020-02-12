extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

struct Square {
    x: i32,
    y: i32,
    value: u32,
}

type State = Vec<Square>;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

fn draw_board(canvas: &mut Canvas<Window>, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas
        .draw_line(Point::new(0, 200), Point::new(800, 200))
        .unwrap();
    canvas
        .draw_line(Point::new(0, 400), Point::new(800, 400))
        .unwrap();
    canvas
        .draw_line(Point::new(0, 600), Point::new(800, 600))
        .unwrap();

    canvas
        .draw_line(Point::new(200, 0), Point::new(200, 800))
        .unwrap();
    canvas
        .draw_line(Point::new(400, 0), Point::new(400, 800))
        .unwrap();
    canvas
        .draw_line(Point::new(600, 0), Point::new(600, 800))
        .unwrap();
}

fn add_square(canvas: &mut Canvas<Window>, square: &Square) {
    let color = if square.value == 2 {
        Color::RGB(0, 0, 255)
    } else if square.value == 4 {
        Color::RGB(255, 0, 0)
    } else if square.value == 8 {
        Color::RGB(0, 255, 0)
    } else {
        Color::RGB(255, 255, 255)
    };
    let x = square.x * 200;
    let y = square.y * 200;

    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x, y, 200, 200)).unwrap();
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
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = &mut window.into_canvas().present_vsync().build().unwrap();
    let mut state = vec![];
    draw_board(canvas, Color::RGB(127, 127, 127));
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let mut direction = Direction::NONE;
        for event in event_pump.poll_iter() {
            direction = match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
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
        state = match direction {
            Direction::UP => up(),
            Direction::DOWN => down(),
            Direction::LEFT => left(),
            Direction::RIGHT => right(),
            Direction::NONE => state,
        };
        draw_board(canvas, Color::RGB(127, 127, 127));
        for square in state.iter() {
            add_square(&mut canvas, square);
        }

        canvas.present();
        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
