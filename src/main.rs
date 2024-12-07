use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::ops::Add;
use std::time::Duration;

pub const GRID_X_SIZE: u32 = 15;
pub const GRID_Y_SIZE: u32 = 15;
pub const CELL_SIZE: u32 = 20;

pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SnakeDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);
impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct GameContext {
    pub snake_parts: Vec<Point>,
    pub game_direction: SnakeDirection,
    pub next_direction: SnakeDirection,
    pub food: Point,
    pub game_state: GameState,
    pub score: i32,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            snake_parts: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            game_direction: SnakeDirection::Right,
            next_direction: SnakeDirection::Right,
            game_state: GameState::Paused,
            food: Point(3, 3),
            score: 0,
        }
    }

    pub fn next_tick(&mut self) {
        match self.game_state {
            GameState::Paused => return,
            GameState::GameOver => return,
            GameState::Playing => (),
        }
        let head_position = self.snake_parts.first().unwrap();
        self.game_direction = self.next_direction;

        let mut next_head_position = match self.game_direction {
            SnakeDirection::Up => *head_position + Point(0, -1),
            SnakeDirection::Down => *head_position + Point(0, 1),
            SnakeDirection::Right => *head_position + Point(1, 0),
            SnakeDirection::Left => *head_position + Point(-1, 0),
        };

        for point in &self.snake_parts {
            if next_head_position.0 == point.0 && next_head_position.1 == point.1 {
                self.game_state = GameState::GameOver;
            }
        }

        // Walls
        if next_head_position.0 >= GRID_X_SIZE as i32 {
            next_head_position.0 = 0;
        }
        if next_head_position.0 < 0 {
            next_head_position.0 = GRID_X_SIZE as i32 - 1;
        }
        if next_head_position.1 >= GRID_Y_SIZE as i32 {
            next_head_position.1 = 0;
        }
        if next_head_position.1 < 0 {
            next_head_position.1 = GRID_Y_SIZE as i32 - 1;
        }

        // Eat food
        if next_head_position.0 == self.food.0 && next_head_position.1 == self.food.1 {
            // Move food
            let mut rng = thread_rng();
            self.food.0 = rng.gen_range(0..GRID_X_SIZE) as i32;
            self.food.1 = rng.gen_range(0..GRID_Y_SIZE) as i32;
            for point in &self.snake_parts {
                if self.food.0 == point.0 {
                    self.food.0 = rng.gen_range(0..GRID_X_SIZE) as i32;
                }

                if self.food.1 == point.1 {
                    self.food.1 = rng.gen_range(0..GRID_Y_SIZE) as i32;
                }
            }

            self.score += 1;

            self.snake_parts.reverse();
            self.snake_parts.push(next_head_position);
            self.snake_parts.reverse();
        } else {
            self.snake_parts.pop();
            self.snake_parts.reverse();
            self.snake_parts.push(next_head_position);
            self.snake_parts.reverse();
        }
    }

    pub fn move_up(&mut self) {
        if self.game_direction != SnakeDirection::Down {
            self.next_direction = SnakeDirection::Up;
        }
    }

    pub fn move_down(&mut self) {
        if self.game_direction != SnakeDirection::Up {
            self.next_direction = SnakeDirection::Down;
        }
    }

    pub fn move_right(&mut self) {
        if self.game_direction != SnakeDirection::Left {
            self.next_direction = SnakeDirection::Right;
        }
    }

    pub fn move_left(&mut self) {
        if self.game_direction != SnakeDirection::Right {
            self.next_direction = SnakeDirection::Left;
        }
    }

    pub fn toggle_pause(&mut self) {
        self.game_state = match self.game_state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            GameState::GameOver => GameState::GameOver,
        }
    }
}

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_background(context);
        self.draw_food(context)?;
        self.draw_player(context)?;
        self.canvas.present();

        Ok(())
    }

    fn draw_background(&mut self, context: &GameContext) {
        let color = match context.game_state {
            GameState::Playing => Color::RGB(0, 0, 0),
            GameState::Paused => Color::RGB(30, 30, 30),
            GameState::GameOver => Color::RGB(60, 0, 0),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
        // self.canvas.set_draw_color(Color::GREEN);
        for (i, point) in context.snake_parts.iter().enumerate() {
            if i % 2 == 0 {
                self.canvas.set_draw_color(Color::RGB(245, 110, 0));
            } else {
                self.canvas.set_draw_color(Color::RGB(202, 91, 0));
            }
            self.draw_dot(point)?;
        }

        Ok(())
    }

    fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        // self.canvas.set_draw_color(Color::RED);
        self.canvas.set_draw_color(Color::RGB(252, 186, 3));
        self.draw_dot(&context.food)?;
        Ok(())
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(sdl2::rect::Rect::new(
            x * CELL_SIZE as i32,
            y * CELL_SIZE as i32,
            CELL_SIZE,
            CELL_SIZE,
        ))?;

        Ok(())
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Rusty Snake",
            GRID_X_SIZE * CELL_SIZE,
            GRID_Y_SIZE * CELL_SIZE,
        )
        .position_centered()
        // .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut renderer = Renderer::new(window)?;
    let mut context = GameContext::new();

    let mut frame_counter = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => context.move_up(),
                    Keycode::A => context.move_left(),
                    Keycode::S => context.move_down(),
                    Keycode::D => context.move_right(),
                    Keycode::Space => context.toggle_pause(),
                    Keycode::Escape => break 'running,
                    _ => {}
                },
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        frame_counter += 1;
        if frame_counter % 6 == 0 {
            context.next_tick();
            frame_counter = 0;
        }

        renderer.draw(&context)?;
        // The rest of the game loop goes here...
    }

    Ok(())
}
