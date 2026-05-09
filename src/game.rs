use std::{thread, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::{
    board::{Board, Cell},
    config::Config,
    snake::{Snake, SnakeDirection},
};

#[derive(Debug)]
pub enum SnakeError {
    WallCollide,
    BodyCollide,
}

#[derive(Debug)]
pub struct SnakeGame {
    board: Board,
    snake: Option<Snake>,
    config: Config,
}

impl SnakeGame {
    pub fn new(board_dim: (usize, usize)) -> Self {
        assert!(board_dim.0 <= 256 && board_dim.1 <= 256);
        Self {
            board: Board::new(board_dim.0, board_dim.1),
            snake: None,
            config: Config::default(),
        }
    }

    pub fn with_config(&mut self, config: Config) {
        self.config = config
    }

    pub fn place_apple(&mut self, n: u16) {
        for _ in 0..n {
            // TODO: improve random apple placement
            let mut idx = rand::random_range(0..self.board.data.len());
            while !matches!(self.board.data[idx], Cell::Empty) {
                idx = rand::random_range(0..self.board.data.len());
            }

            self.board.data[idx] = Cell::Apple;
        }
    }

    pub fn handle_input(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;

        loop {
            let Ok(true) = event::poll(Duration::from_millis(0)) else {
                break;
            };

            let Ok(Event::Key(key)) = event::read() else {
                break;
            };

            match key.code {
                KeyCode::Char('w') | KeyCode::Char('k') => {
                    self.snake
                        .as_mut()
                        .map(|snake| snake.redirect(SnakeDirection::Up));
                }

                KeyCode::Char('s') | KeyCode::Char('j') => {
                    self.snake
                        .as_mut()
                        .map(|snake| snake.redirect(SnakeDirection::Down));
                }

                KeyCode::Char('a') | KeyCode::Char('h') => {
                    self.snake
                        .as_mut()
                        .map(|snake| snake.redirect(SnakeDirection::Left));
                }

                KeyCode::Char('d') | KeyCode::Char('l') => {
                    self.snake
                        .as_mut()
                        .map(|snake| snake.redirect(SnakeDirection::Right));
                }

                _ => {}
            }

            // Drain Buffered Inputs
            // while event::poll(Duration::from_millis(0)).unwrap_or(false) {
            //     let _ = event::read();
            // }

            break;
        }

        disable_raw_mode()
    }

    pub fn render(&mut self) {
        let Some(snake) = &self.snake else { return };

        for idx in &snake.body {
            self.board.data[*idx] = Cell::Snake;
        }

        _ = self.board.render();

        println!(
            "\n\t\t\t{}: {}",
            "Score".bold(),
            format!("{}", snake.eaten).bold().yellow()
        );
    }

    pub fn start(&mut self) {
        let mut err: Option<SnakeError> = None;

        let (width, height) = self.board.dimension();
        let starting_pos = rand::random_range(0..(width * height));

        self.snake = Some(Snake::new(starting_pos));
        self.place_apple(1);

        loop {
            println!("\x1b[2J\x1b[H");

            if let Err(err) = self.handle_input() {
                panic!("an error occurred during input handling: {err:?}");
            }

            if let Some(snake) = &mut self.snake {
                match snake.crawl(&mut self.board) {
                    Ok(true) => self.place_apple(1),
                    Err(error) => err = Some(error),
                    _ => {}
                }
            }

            self.render();
            thread::sleep(Duration::from_millis(150));

            let Some(err) = err else {
                continue;
            };

            match err {
                SnakeError::WallCollide => {
                    println!("\t\t\t{}", "YOU HIT THE WALL!".bold().red());
                    break;
                }

                SnakeError::BodyCollide => {
                    println!("\t\t\t{}", "YOU BIT YOURSELF!".bold().red());
                    break;
                }
            }
        }
    }
}
