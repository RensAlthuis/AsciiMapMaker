#![allow(dead_code)]

extern crate termion;
extern crate byteorder;

mod map;
mod graphics;
mod menu;
mod keymap;

use graphics::{Graphics, Event, Drawable, Token};
use graphics::terminal::TermGraphics;
use menu::Menu;
use map::{Window, MapRef};
use keymap::KeyMap;

struct Context{
    window : Window,
    menu : Menu,
    map : MapRef,
}

fn main() {

    let mut output = TermGraphics::new();
    let (width, height) = output.size();

    let m = MapRef::new(map::Map::from_file("test.map"));

    let options = vec!["hello", "cheese", "Something else", "another thing"];
    let menu = Menu::new(options);
    let window = Window::new(m.clone(), 0, 0, width - menu.width() - 3, height - 3);
    let window_border = graphics::Square::new(Token::Character('*'), width - menu.width() - 1, height-1);

    let mut keymap = KeyMap::<Context>::new();
    register_keys(&mut keymap);


    output.draw(&window, 1, 1)
          .draw(&window_border, 0, 0)
          .draw(&menu, (width-menu.width()) as isize, 0);

    let mut context = Context {
        menu,
        window,
        map : m.clone(),
    };

    let run_func = |output : &mut TermGraphics,
                    keymap : &mut KeyMap<Context>,
                    ev,
                    context : &mut Context |
    {
            let (window_x, window_y) = (1, 1);
            let (width, _height) = output.size();

            match ev {
                Event::Key(c) => {
                    match c {
                        'q' => {return false;},
                        c => {
                            keymap.call(c, context);
                        }
                    }

                    let menu = &mut context.menu;
                    output.draw(&mut context.window, window_x, window_y);
                    output.draw(menu, (width-menu.width()) as isize, 0);
                }

                Event::MouseDown(b, x, y) => {
                    let window = &mut context.window;
                    match b {
                        0 => {
                            window.set( x as isize - window_x as isize,
                                        y as isize - window_y as isize,
                                        Token::Tile(2));
                        },
                        1 => {
                            window.set( x as isize - window_x as isize,
                                        y as isize - window_y as isize,
                                        Token::Style(graphics::StyleType::Colour{r: 255, g:0, b:0}, Box::new(Token::Tile(3))));
                        },
                        2 => {
                            window.set( x as isize - window_x as isize,
                                        y as isize - window_y as isize,
                                        Token::Style(graphics::StyleType::Colour{r: 255, g:0, b:255}, Box::new(Token::Tile(3))));
                        }
                        _ => ()
                    }
                    output.draw(window, window_x, window_y);
                },
                Event::MouseDrag(b, x, y) => {
                    let window = &mut context.window;
                    match b {
                        0 => {
                            window.set( x as isize - window_x as isize,
                                        y as isize - window_y as isize,
                                        Token::Tile(2));
                        },
                        1 => {
                            println!("key 1");
                            window.set( x as isize - window_x as isize,
                                        y as isize - window_y as isize,
                                        Token::Tile(2));
                                        // Token::Style(graphics::StyleType::Colour{r: 255, g:255, b:0}, Box::new(Token::Tile(3))));
                        }
                        _ => ()
                    }
                    output.draw(window, window_x, window_y);
                }
                _ => ()
            }
            true
    };

    output.run(&mut keymap, &mut context, run_func);
}


fn register_keys(keymap : &mut KeyMap<Context>){
    keymap.register('h', |context| { context.window.scroll(-1, 0);});
    keymap.register('l', |context| { context.window.scroll(1, 0);});
    keymap.register('k', |context| { context.window.scroll(0, -1);});
    keymap.register('j', |context| { context.window.scroll(0, 1);});
    keymap.register('x', |context| { context.menu.up();});
    keymap.register('z', |context| { context.menu.down();});
    keymap.register('\n',|context| { context.menu.toggle();});
    keymap.register('r', |context| {
        let newmap = map::Map::new(100, 100);
        newmap.to_file("test.map");
        context.map = MapRef::new(newmap);
        context.window = Window::new(context.map.clone(), 0, 0, context.window.width, context.window.height);

    });
    keymap.register('s', |context| {context.map.to_file("test.map");});
}