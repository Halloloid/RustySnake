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

    let inside_boundary = Boundary{
        start:Position { x: boundary.start.x+1, y: boundary.start.y+1 },
        end:Position { x: boundary.end.x-2, y: boundary.end.y-2 },
    };

    let snake = Snake{
        body: vec![Position{x:2,y:10},Position{x:3,y:10}],
        direction:Direction::Right,
    };

    let mut food_postion = Position{x:0,y:0};

    terminal::enable_raw_mode()?;
    create_boundary(&mut stdout, &boundary)?;
    // for _ in 1..100{
    //     spawn_food(&mut stdout, &mut food_postion, &inside_boundary)?;
    // }

    draw_snake(&mut stdout,&snake.body)?;
    terminal::disable_raw_mode()?;

    stdout.execute(cursor::MoveTo(0,boundary.end.y))?;

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

fn spawn_food(stdout: &mut Stdout,food_postion:&mut Position,boundary: &Boundary) -> io::Result<()> {
    *food_postion = Position { 
        x: rand::thread_rng().gen_range(boundary.start.x..boundary.end.x), 
        y: rand::thread_rng().gen_range(boundary.start.y..boundary.end.y)
    };

    stdout
    .queue(cursor::MoveTo(food_postion.x,food_postion.y))?
    .queue(style::PrintStyledContent("⏹".magenta()))?;

    stdout.flush()?;
    Ok(())
}

fn draw_snake(stdout: &mut Stdout,body: &Vec<Position>)-> io::Result<()>{
    for pos in body{
        stdout
        .queue(cursor::MoveTo(pos.x,pos.y))?
        .queue(style::PrintStyledContent("⏹".yellow()))?;
    }

    stdout.flush()?;
    Ok(())
}
