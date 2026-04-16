use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode},
    style::{self, Stylize},
    terminal::{self,EnterAlternateScreen,LeaveAlternateScreen},
};

use rand::Rng;
use std::io::{self, Stdout, Write, stdout};
use std::{thread, time::Duration};

#[derive(Clone, Copy,PartialEq)]
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
    score : i32
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
            x: boundary.start.x + 2,
            y: boundary.start.y + 2,
        },
        end: Position {
            x: boundary.end.x - 2,
            y: boundary.end.y - 2,
        },
    };

    let snake = Snake {
        body: vec![Position { x: 3, y: 10 }, Position { x: 2, y: 10 }],
        direction: Direction::Right,
    };

    let mut game = Game {
        snake: snake,
        food: Position { x: 0, y: 0 },
        running: true,
        score : 0
    };

    terminal::enable_raw_mode()?;
    
    stdout.execute(EnterAlternateScreen)?;
    spawn_food(&mut stdout, &mut game.food, &inside_boundary)?;
    while game.running {
        move_snake(&mut game.snake)?;
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;
        create_boundary(&mut stdout, &boundary)?;
        draw_snake(&mut stdout, &game.snake.body)?;
        draw_food(&mut stdout, &mut game)?;
        check_wall_collision( &mut game, &boundary)?;
        check_food_collision(&mut stdout, &mut game, &inside_boundary)?;

        stdout.flush()?;
            thread::sleep(Duration::from_millis(80));
    }
    
    stdout.execute(LeaveAlternateScreen)?;
    let msg = format!("You are Out\tScore is {}",game.score);
    stdout
        .queue(cursor::MoveTo(0, boundary.end.y))?
        .queue(style::PrintStyledContent(msg.magenta()))?;
    stdout.flush()?;

    terminal::disable_raw_mode()?;

    stdout.execute(cursor::MoveTo(0, boundary.end.y+2))?;

    Ok(())
}

fn create_boundary(stdout: &mut Stdout, boundary: &Boundary) -> io::Result<()> {
    for y in boundary.start.y..boundary.end.y {
        for x in boundary.start.x..boundary.end.x {
            if (y == 0 || y == boundary.end.y - 1) || (x == 0 || x == boundary.end.x - 1) {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent("██".red()))?;
            }
        }
    }
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
        .queue(style::PrintStyledContent("██".magenta()))?;

    stdout.flush()?;
    Ok(())
}

fn draw_snake(stdout: &mut Stdout, body: &Vec<Position>) -> io::Result<()> {
    for pos in body {
        stdout
            .queue(cursor::MoveTo(pos.x, pos.y))?
            .queue(style::PrintStyledContent("██".yellow()))?;
    }
    Ok(())
}

fn move_snake(snake: &mut Snake) -> io::Result<()> {
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

    Ok(())
}

fn check_wall_collision(
    game: &mut Game,
    boundary: &Boundary,
) -> io::Result<()> {
    let head = &game.snake.body[0];
    if (head.x == boundary.start.x+1 || head.x == boundary.end.x-2)
        || (head.y == boundary.start.y+1 || head.y == boundary.end.y-2)
    {
        game.running = false;
    }
    Ok(())
}

fn check_food_collision(stdout: &mut Stdout,game: &mut Game,inside_boundary:&Boundary) -> io::Result<()> {
    let head = game.snake.body[0].clone();

    if head == game.food{
        game.snake.body.insert(0, game.food);
        game.score += 1 ;
        spawn_food(stdout, &mut game.food, &inside_boundary)?;
    }

    Ok(())
}

fn draw_food(stdout: &mut Stdout,game: &mut Game) -> io::Result<()> {

    stdout
    .queue(cursor::MoveTo(game.food.x,game.food.y))?
    .queue(style::PrintStyledContent("██".magenta()))?;

    Ok(())
}