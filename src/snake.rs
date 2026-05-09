use std::collections::VecDeque;

use crate::{
    board::{Board, Cell},
    config::Config,
    game::SnakeError,
};

#[derive(Debug)]
pub struct Snake {
    pub(crate) body: VecDeque<usize>,
    pub(crate) eaten: usize,

    direction: SnakeDirection,
}

impl Snake {
    pub fn new(starting_pos: usize) -> Self {
        Self {
            body: vec![starting_pos].into(),
            eaten: 0,
            direction: SnakeDirection::None,
        }
    }

    pub fn redirect(&mut self, direction: SnakeDirection) {
        // Only re-direct if the new direction is not the exact
        // opposite of the current direction.
        // e.g. up -> down, right -> left, etc.
        if (self.direction as i8).abs() != (direction as i8).abs() {
            self.direction = direction
        }
    }

    pub fn try_eat(&mut self, cell: &mut Cell) -> Result<bool, SnakeError> {
        match &cell {
            Cell::Empty => Ok(false),
            Cell::Snake => Err(SnakeError::BodyCollide),
            Cell::Apple => Ok((self.eaten += 1, true).1),
        }
    }

    pub fn crawl(&mut self, config: &Config, board: &mut Board) -> Result<bool, SnakeError> {
        assert!(!self.body.is_empty(), "headless body!");

        let Config { wall_collisions } = &config;

        let head = *self.body.front().unwrap();
        let (width, height) = board.dimension();
        let (row, col) = (head / width, head % height);

        let (collide, mut next_head, wrap_pos) = match self.direction {
            SnakeDirection::Up => (
                row <= 0,
                head.saturating_sub(width),
                (height - 1) * width + col,
            ),

            SnakeDirection::Down => (row >= height - 1, head.saturating_add(width), col),

            SnakeDirection::Left => (col <= 0, head.saturating_sub(1), head + (width - 1)),

            SnakeDirection::Right => (col >= width - 1, head.saturating_add(1), head - col),

            _ => (false, head, head),
        };

        if collide {
            if *wall_collisions {
                Err(SnakeError::WallCollide)?
            }

            next_head = wrap_pos;
        }

        if head == next_head {
            return Ok(false);
        }

        let ate = self.try_eat(&mut board.data[next_head])?;
        self.body.push_front(next_head);

        if self.body.len() > self.eaten + 1 {
            board.data[self.body.pop_back().unwrap()] = Cell::Empty
        }

        Ok(ate)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SnakeDirection {
    None,
    Up = -1,
    Down = 1,
    Left = -2,
    Right = 2,
}
