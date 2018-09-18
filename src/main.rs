extern crate piston_window;

use piston_window::*;

struct Snake
{
    x : u32,
    y : u32
}

impl Snake
{

}

fn main()
{
    let mut window: PistonWindow =
        WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();

    let block_size = 32;

    let s = Snake{x : 2, y : 2};

    while let Some(e) = window.next()
    {
        window.draw_2d(&e, |c, g| 
        {
            clear([0.0, 0.0, 0.0, 1.0], g);
            rectangle([0.0, 1.0, 0.0, 1.0], // red
                      [s.x as f64 * block_size as f64, s.y as f64 * block_size as f64, block_size as f64, block_size as f64], // rectangle
                      c.transform, g);
        });
    }
}