use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode},
    style::{self, Stylize},
    terminal,
};

use rand::Rng;
use std::io::{self, Stdout, Write, stdout};
use std::{thread, time::Duration};

struct Position {
    x: u16,
    y: u16,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Snake {
    body: Vec<Position>,
    direction: Direction,
}

struct Game {
    snake: Snake,
    food: Position,
    running: bool,
}

struct Boundary {
    start: Position,
    end: Position,
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let boundary = Boundary {
        start: Position { x: 0, y: 0 },
        end: Position { x: 50, y: 20 },
    };

    terminal::enable_raw_mode()?;
    create_boundary(&mut stdout, boundary)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn create_boundary(stdout: &mut Stdout, boundary: Boundary) -> io::Result<()> {
    for y in boundary.start.y..boundary.end.y {
        for x in boundary.start.x..boundary.end.x {
            if (y == 0 || y == boundary.end.y - 1) || (x == 0 || x == boundary.end.x - 1) {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("██".red()))?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
}
