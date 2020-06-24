#![allow(dead_code)]

extern crate termion;
extern crate byteorder;

mod map;
mod graphics;
mod menu;

use graphics::{Graphics, Event};
use graphics::terminal::TermGraphics;

fn main() {

    let mut output = TermGraphics::new();
    let m = map::Map::new(30, 30);
    m.to_file("test.map");
    let m = std::rc::Rc::new(map::Map::from_file("test.map"));
    let window = map::window::Window::new(&m, 0, 0, 10, 10);
    m.set(1, 1, 1);
    m.set(2, 2, 1);
    m.set(1, 2, 1);
    m.set(2, 1, 1);

    output.clear()
          .draw(&window, 0, 0);

    let square = graphics::square::Square::filled(1, 1, 5, 5);
    output.run(|output, ev| {
        match ev {
            Event::Key('q') => {
                false
            },
            Event::MouseDrag(x, y) => {
                output.draw(&square, (x as i16)-(window.width/2) as i16,(y as i16)-(window.height/2) as i16);
                true
            }
            _ => true
        }
    });

}