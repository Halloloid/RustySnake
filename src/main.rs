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

#[derive(Clone, Copy)]
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
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let boundary = Boundary {
        start: Position { x: 0, y: 0 },
        end: Position { x: 50, y: 20 },
    };

    let inside_boundary = Boundary {
        start: Position {
            x: boundary.start.x + 1,
            y: boundary.start.y + 1,
        },
        end: Position {
            x: boundary.end.x - 2,
            y: boundary.end.y - 2,
        },
    };

    let snake = Snake {
        body: vec![Position { x: 2, y: 10 }, Position { x: 3, y: 10 }],
        direction: Direction::Right,
    };

    let mut game = Game {
        snake: snake,
        food: Position { x: 0, y: 0 },
        running: true,
    };

    terminal::enable_raw_mode()?;
    // for _ in 1..100 {
    //     spawn_food(&mut stdout, &mut game.food, &inside_boundary)?;
    //     create_boundary(&mut stdout, &boundary)?;
    //     move_snake(&mut stdout, &mut game.snake)?;
    //     draw_snake(&mut stdout, &game.snake.body)?;
    // }

    while game.running {
        create_boundary(&mut stdout, &boundary)?;
        move_snake(&mut stdout, &mut game.snake)?;
        draw_snake(&mut stdout, &game.snake.body)?;
        check_wall_collision( &mut game, &boundary)?;
    }
    stdout
        .queue(cursor::MoveTo(0, boundary.end.y))?
        .queue(style::PrintStyledContent("You are Out".magenta()))?;
    stdout.flush()?;

    terminal::disable_raw_mode()?;

    stdout.execute(cursor::MoveTo(0, boundary.end.y+1))?;

    Ok(())
}

fn create_boundary(stdout: &mut Stdout, boundary: &Boundary) -> io::Result<()> {
    for y in boundary.start.y..boundary.end.y {
        for x in boundary.start.x..boundary.end.x {
            if (y == 0 || y == boundary.end.y - 1) || (x == 0 || x == boundary.end.x - 1) {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("⏹".red()))?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
}

fn spawn_food(
    stdout: &mut Stdout,
    food_postion: &mut Position,
    boundary: &Boundary,
) -> io::Result<()> {
    *food_postion = Position {
        x: rand::thread_rng().gen_range(boundary.start.x..boundary.end.x),
        y: rand::thread_rng().gen_range(boundary.start.y..boundary.end.y),
    };

    stdout
        .queue(cursor::MoveTo(food_postion.x, food_postion.y))?
        .queue(style::PrintStyledContent("⏹".magenta()))?;

    stdout.flush()?;
    Ok(())
}

fn draw_snake(stdout: &mut Stdout, body: &Vec<Position>) -> io::Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    for pos in body {
        stdout
            .queue(cursor::MoveTo(pos.x, pos.y))?
            .queue(style::PrintStyledContent("⏹".yellow()))?;
    }

    stdout.flush()?;
    Ok(())
}

fn move_snake(stdout: &mut Stdout, snake: &mut Snake) -> io::Result<()> {
    let mut direction = snake.direction;
    if event::poll(Duration::from_millis(0))? {
        if let Event::Key(key_event) = event::read()? {
            direction = match key_event.code {
                KeyCode::Up => Direction::Up,
                KeyCode::Down => Direction::Down,
                KeyCode::Left => Direction::Left,
                KeyCode::Right => Direction::Right,
                _ => direction,
            };
        }
    }

    match direction {
        Direction::Up => {
            snake.body.insert(
                0,
                Position {
                    x: snake.body[0].x,
                    y: snake.body[0].y.saturating_sub(1),
                },
            );
        }
        Direction::Down => {
            snake.body.insert(
                0,
                Position {
                    x: snake.body[0].x,
                    y: snake.body[0].y.saturating_add(1),
                },
            );
        }
        Direction::Right => {
            snake.body.insert(
                0,
                Position {
                    x: snake.body[0].x.saturating_add(1),
                    y: snake.body[0].y,
                },
            );
        }
        Direction::Left => {
            snake.body.insert(
                0,
                Position {
                    x: snake.body[0].x.saturating_sub(1),
                    y: snake.body[0].y,
                },
            );
        }
    }
    snake.direction = direction;
    snake.body.pop();
    stdout.flush()?;
    thread::sleep(Duration::from_millis(50));
    Ok(())
}

fn check_wall_collision(
    game: &mut Game,
    boundary: &Boundary,
) -> io::Result<()> {
    let head = &game.snake.body[0];
    if (head.x == boundary.start.x || head.x == boundary.end.x)
        || (head.y == boundary.start.y || head.y == boundary.end.y)
    {
        game.running = false;
    }
    Ok(())
}
