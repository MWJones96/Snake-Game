extern crate piston_window;

use piston_window::*;

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction
{
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone)]
pub struct Piece(u32, u32);

pub struct Snake
{
    body: LinkedList<Piece>,
    length: u32,
    dir: Direction
}

impl Snake
{
    fn pressed(&mut self, btn: &Button)
    {
        let last_direction = self.dir.clone();

        self.dir = match btn
        {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        }
    }

    fn update(&mut self)
    {
        let mut new_head: Piece = (*self.body.front().expect("No Head Found")).clone();

        match self.dir
        {
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1
        }

        self.body.pop_back();
        self.body.push_front(new_head);
    }
}

fn main() 
{
    const WIDTH: u32 = 30;
    const HEIGHT: u32 = 20;
    const BLOCK_SIZE: u32 = 25;

    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [WIDTH * BLOCK_SIZE, HEIGHT * BLOCK_SIZE])
            .build().unwrap();

    let mut snake = Snake
    {
        body: LinkedList::from_iter((vec![Piece(WIDTH / 2, HEIGHT / 2), Piece(WIDTH / 2 + 1, HEIGHT / 2), Piece(WIDTH / 2 + 2, HEIGHT / 2)]).into_iter()),
        length: BLOCK_SIZE,
        dir: Direction::Left
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) 
    {
        if let Some(k) = e.button_args() 
        {
            if k.state == ButtonState::Press 
            {
                snake.pressed(&k.button);
            }
        }

        if let Some(u) = e.update_args() 
        {
            snake.update();
        }

            window.draw_2d(&e, |c, g| 
            {
                clear([0.0, 0.0, 0.0, 1.0], g);

                let snake_blocks: Vec<types::Rectangle> = snake.body
                    .iter()
                    .map(|p| Piece(p.0 * BLOCK_SIZE, p.1 * BLOCK_SIZE))
                    .map(|p| rectangle::square(p.0 as f64, p.1 as f64, snake.length as f64))
                    .collect();

                snake_blocks
                    .into_iter()
                    .for_each(|square| rectangle([1.0, 1.0, 1.0, 1.0], square, c.transform, g));
            });;
    }
}