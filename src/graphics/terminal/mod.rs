mod character_map;

use std::io::{stdin, stdout, Write};
use termion::cursor::HideCursor;
use termion::screen::AlternateScreen;
use termion::event::{Event, Key, MouseEvent, MouseButton};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use super::{Graphics, Drawable, token::Token, StyleType};

pub type Output = MouseTerminal<HideCursor<AlternateScreen<RawTerminal<std::io::Stdout>>>>;

pub struct TermGraphics {
    stdout: Option<Output>,
    last_button_down : usize,
}

impl TermGraphics {
    pub fn new() -> Self {
        let stdout = stdout()
            .into_raw_mode()
            .ok()
            .map(|e| AlternateScreen::from(e))
            .map(|e| HideCursor::from(e))
            .map(|e| MouseTerminal::from(e));

        TermGraphics { stdout , last_button_down : 0}
    }

    pub fn clear(&mut self) -> &mut Self {
        if let Some(output) = &mut self.stdout {
            write!(output, "{}",termion::clear::All).unwrap();

        }
        self
    }

    pub fn goto(&mut self, x: u16, y: u16) -> &mut Self {
        let (w, h) = termion::terminal_size().unwrap();

        let x = x+1;
        let y = y+1;
        let x = if x > w { w } else { x };
        let y = if y > h { h } else { y };

        if let Some(output) = &mut self.stdout {
            write!(output, "{}", termion::cursor::Goto(x, y)).unwrap();
        }
        self
    }

    pub fn put(&mut self, c: char) -> &mut Self {
        if let Some(output) = &mut self.stdout {
            write!(output, "{}", c).unwrap();
        }
        self
    }

    pub fn set_style(&mut self, style : StyleType) -> &mut Self {
        //TODO implement other types
        if let Some(output) = &mut self.stdout {
            match style {
                StyleType::Underline => {
                    write!(output, "{}", termion::style::Underline).unwrap();
                },
                StyleType::NoUnderline => {
                    write!(output, "{}", termion::style::NoUnderline).unwrap();
                },
                StyleType::Colour{r,g,b} => {
                    write!(output, "{}", termion::color::Fg(termion::color::Rgb(r,g,b))).unwrap();
                }
                StyleType::ResetColour => {
                    write!(output, "{}", termion::color::Fg(termion::color::White)).unwrap();
                }
                _ => ()
            }
        }
        self
    }

    pub fn down(&mut self, n: u16) -> &mut Self {
        if let Some(output) = &mut self.stdout {
            write!(output, "{}", termion::cursor::Down(n)).unwrap();
        }
        self
    }

    pub fn right(&mut self, n: u16) -> &mut Self {
        if let Some(output) = &mut self.stdout {
            write!(output, "{}", termion::cursor::Right((n) as u16)).unwrap();
        }
        self
    }

    pub fn left(&mut self, n: u16) -> &mut Self {
        if let Some(output) = &mut self.stdout {
            write!(output, "{}", termion::cursor::Left((n) as u16)).unwrap();
        }
        self
    }

    pub fn flush(&mut self) -> &mut Self {
        if let Some(output) = &mut self.stdout {
            output.flush().unwrap();
        }
        self
    }

    pub fn draw_token(&mut self, x : isize, y: isize, token : Token) -> &mut Self{
        match token {
            Token::Tile(t) => {
                self.goto(x as u16, y as u16)
                    .put(character_map::map(t))
            },
            Token::Character(c) => {
                self.goto(x as u16, y as u16)
                    .put(c)
            },
            Token::StyleType(s) => {
                self.goto(x as u16, y as u16)
                    .set_style(s)
            },
            Token::Style(s, t) => {
                self.set_style(s)
                    .draw_token(x, y, *t)
                    .set_style(s.invert())
            }
            Token::Empty => { self.right(1) }
        }
    }
}

impl Graphics for TermGraphics {
    fn run<F>(&mut self, keymap : &mut crate::KeyMap<F>, context : &mut F, f: fn(&mut Self, &mut crate::KeyMap<F>, super::Event, &mut F) -> bool)
    {
        let input = stdin();
        for c in input.events() {
            let evt = c.unwrap();
            let res = match evt {
                Event::Key(Key::Char(key)) => f(self, keymap,  super::Event::Key(key), context),
                Event::Mouse(me) => match me {
                    MouseEvent::Press(b, x, y) => {
                        self.last_button_down = from(b);
                        f(self, keymap, super::Event::MouseDown(self.last_button_down, x as usize -1, y as usize -1), context)
                    },
                    MouseEvent::Hold(x, y) => f(self, keymap, super::Event::MouseDrag(self.last_button_down, x as usize -1, y as usize -1), context),
                    MouseEvent::Release(x, y) => f(self, keymap, super::Event::MouseUp(x as usize -1, y as usize -1), context),
                },
                _ => true,
            };

            if res == false {
                break;
            }

            self.flush();
        }
    }

    fn draw<'a>(&mut self, d: &'a impl Drawable<'a>, x: isize, y: isize) -> &mut Self {

        let size = termion::terminal_size().unwrap();
        let (w, h) = (size.0 as isize, size.1 as isize);

        for (col, row, token) in d.iter() {

            let px = x + col as isize;
            let py = y + row as isize;

            if px < 0 || px > w || py < 0 || py > h {
                continue;
            };

            self.draw_token(px, py, token);
        }

        self.flush()
    }

    fn size(&self) -> (usize, usize) {
        let (w, h) = termion::terminal_size().unwrap();
        (w as usize, h as usize)
    }

}


//TODO: turn this into an enum.
fn from(button: MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::WheelUp => 3,
        MouseButton::WheelDown => 4,
    }
}