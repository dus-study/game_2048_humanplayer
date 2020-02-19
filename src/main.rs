extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};
use std::time::Duration;

struct Square {
    x: i32,
    y: i32,
    value: u32,
}

type State = Vec<Square>;

struct View {
    canvas: Canvas<Window>,
    lines: Vec<(Point, Point)>,
    bg_color: Color,
    line_color: Color,
    squares: Vec<Square>,
    // font:
}

impl View {
    fn new(
        sdl_context: &Sdl,
        bg_color: Color,
        line_color: Color,
        _game_size: u8,
        window_size: u32,
    ) -> View {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", window_size, window_size)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let mut lines: Vec<(Point, Point)> = vec![];
        let window_size = window_size as i32;
        lines.push((Point::new(0, 200), Point::new(window_size, 200)));
        lines.push((Point::new(0, 400), Point::new(window_size, 400)));
        lines.push((Point::new(0, 600), Point::new(window_size, 600)));
        lines.push((Point::new(200, 0), Point::new(200, window_size)));
        lines.push((Point::new(400, 0), Point::new(400, window_size)));
        lines.push((Point::new(600, 0), Point::new(600, window_size)));

        // let ttf_context = sdl2::ttf::init().unwrap();
        // let font = ttf_context.load_font("fonts/DejaVuSansMono-Bold.ttf", 128).unwrap();

        View {
            canvas,
            lines,
            bg_color,
            line_color,
            squares: vec![],
        }
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
        self.canvas.set_draw_color(self.line_color);
        for line in self.lines.iter() {
            self.canvas.draw_line(line.0, line.1).unwrap();
        }
        for square in self.squares.iter() {
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

            self.canvas.set_draw_color(color);
            self.canvas.fill_rect(Rect::new(x, y, 200, 200)).unwrap();
            // let surface = font.render(square.value as str).unwrap();
        }
        self.canvas.present();
    }

    fn update(&mut self, squares: Vec<Square>) {
        self.squares = squares;
    }
}

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
