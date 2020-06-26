#![allow(dead_code)]

extern crate termion;
extern crate byteorder;

mod map;
mod graphics;
mod menu;

use graphics::{Graphics, Event, Drawable};
use graphics::terminal::TermGraphics;
use menu::Menu;

fn main() {

    let mut output = TermGraphics::new();
    let (width, height) = output.size();

    let m = std::rc::Rc::new(map::Map::from_file("test.map"));

    let options = vec!["hello", "cheese", "Something else", "another thing"];
    let mut menu = Menu::new(options);
    let mut window = map::window::Window::new(&m, 0, 0, width - menu.width() - 1, height - 1);

    let (window_x, window_y) = (1, 1);


    output.draw(&window, window_x, window_y)
          .draw(&menu, (width-menu.width()) as isize, 0);


    output.run(
        &mut |output : &mut TermGraphics, ev| {
            match ev {
                Event::Key(c) => {
                    match c {
                        'q' => return false,
                        'h' => { window.scroll(-1, 0);},
                        'l' => { window.scroll(1, 0);},
                        'k' => { window.scroll(0, -1);},
                        'j' => { window.scroll(0, 1);},
                        's' => { m.to_file("test.map");}
                        'r' => {
                            let newmap = map::Map::new(100, 100);
                            newmap.to_file("test.map");
                        }
                        'x' => { menu.up();}
                        'z' => { menu.down();}
                        c => {
                            output.draw(&std::format!("Key Pressed: {}", c), 0 ,0);
                        }
                    }

                    output.draw(&window, window_x, window_y)
                          .draw(&menu, (width-menu.width()) as isize, 0);
                }

                Event::MouseDown(_, x, y) => {
                    window.set(x as isize - window_x as isize,
                               y as isize - window_y as isize,
                               2);
                    output.draw(&window, window_x, window_y);
                },
                Event::MouseDrag(x, y) => {
                    window.set(x as isize - window_x as isize,
                               y as isize - window_y as isize,
                               2);
                    output.draw(&window, window_x, window_y);
                }
                _ => ()
            }
            true
        }
    );
}