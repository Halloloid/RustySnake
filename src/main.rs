use crossterm::{
    ExecutableCommand,QueueableCommand,cursor,
    event::{self,Event,KeyCode},
    style::{self,Stylize},
    terminal,
};

use std::io::{self,Write,stdout};
use std::{thread,time::Duration};
use rand::Rng;

struct  Position{
    x:u16,
    y:u16,
}

enum Direction {
    Up,
    Down,
    Right,
    Left
}

struct Snake{
    body:Vec<Position>,
    direction:Direction,
}

struct Game{
    snake:Snake,
    food:Position,
    running:bool,
}

struct Boundary{
    start:Position,
    end:Position,
}

fn main() {
    println!("Hello, world!");
}
