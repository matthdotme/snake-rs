// src/main.rs
mod game;

use crate::game::{Game, Direction};
use crossterm::{ExecutableCommand, terminal::{self, ClearType}, cursor, event::{self, KeyCode}};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

fn render(game: &Game) {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    for y in 0..game.height {
        for x in 0..game.width {
            if game.snake.body.contains(&game::Point { x, y }) {
                print!("S");
            } else if game.food == (game::Point { x, y }) {
                print!("F");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    stdout.flush().unwrap();
}

fn main() {

    let width = 20;
    let height = 20;
    let mut game = Game::new(width, height);

    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    terminal::enable_raw_mode().unwrap();
    stdout().execute(terminal::EnterAlternateScreen).unwrap();

    'gameloop: loop {
        if last_tick.elapsed() >= tick_rate {
            game.update();
            render(&game);
            if game.is_game_over() {
                break 'gameloop;
            }
            last_tick = Instant::now();
        }

        if event::poll(Duration::from_millis(1)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Up => game.snake.change_direction(Direction::Up),
                    KeyCode::Down => game.snake.change_direction(Direction::Down),
                    KeyCode::Left => game.snake.change_direction(Direction::Left),
                    KeyCode::Right => game.snake.change_direction(Direction::Right),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
    stdout().execute(terminal::LeaveAlternateScreen).unwrap();
}
