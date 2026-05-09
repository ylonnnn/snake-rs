use std::{fmt::Display, io::Write};

use crossterm::style::{Color, Stylize};

#[derive(Debug)]
pub struct Board {
    pub(crate) data: Box<[Cell]>,
    dimension: (usize, usize),
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Cell::Empty; width * height].into(),
            dimension: (width, height),
        }
    }

    pub fn dimension(&self) -> (usize, usize) {
        self.dimension
    }

    pub fn render(&self) -> std::io::Result<()> {
        for (i, cell) in self.data.iter().enumerate() {
            print!("{}", &cell);
            std::io::stdout().flush()?;

            if (i + 1) % (self.dimension().0) == 0 {
                println!();
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Apple,
    Snake,
}

impl Cell {
    pub fn color(&self) -> Color {
        match &self {
            Self::Empty => Color::Rgb {
                r: 10,
                g: 175,
                b: 20,
            },

            Self::Apple => Color::Rgb {
                r: 175,
                g: 30,
                b: 30,
            },

            Self::Snake => Color::Rgb { r: 0, g: 80, b: 5 },
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = self.color();
        write!(
            f,
            "{}{}{}",
            "\x1b[1m",
            "   ".with(color).on(color),
            "\x1b[0m"
        )
    }
}
