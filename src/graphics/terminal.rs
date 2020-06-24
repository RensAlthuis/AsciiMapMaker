use std::io::{stdin, stdout, Write};
use termion::cursor::HideCursor;
use termion::screen::AlternateScreen;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};

use super::{Graphics, Drawable};
pub type Output = MouseTerminal<HideCursor<AlternateScreen<RawTerminal<std::io::Stdout>>>>;

pub struct TermGraphics {
    stdout: Option<Output>,
}

impl TermGraphics {
    pub fn new() -> Self {
        let stdout = stdout()
            .into_raw_mode()
            .ok()
            .map(|e| AlternateScreen::from(e))
            .map(|e| HideCursor::from(e))
            .map(|e| MouseTerminal::from(e));

        TermGraphics { stdout }
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
}

impl Graphics for TermGraphics {
    fn run<F>(&mut self, f: F)
    where
        F: Fn(&mut Self, super::Event) -> bool,
    {
        let input = stdin();
        for c in input.events() {
            let evt = c.unwrap();
            let res = match evt {
                Event::Key(Key::Char(key)) => f(self, super::Event::Key(key)),
                Event::Mouse(me) => match me {
                    MouseEvent::Press(_b, x, y) => f(self, super::Event::MouseDown(0, x, y)),
                    MouseEvent::Hold(x, y) => f(self, super::Event::MouseDrag(x, y)),
                    MouseEvent::Release(x, y) => f(self, super::Event::MouseUp(x, y)),
                },
                _ => true,
            };

            if res == false {
                break;
            }

            self.flush();
        }
    }

    fn draw<'a>(&mut self, d: &'a impl Drawable<'a>, x: i16, y: i16) -> &mut Self {

        let size = termion::terminal_size().unwrap();
        let (w, h) = (size.0 as i16, size.1 as i16);

        let mut col = x;
        let mut row = y;
        let iterator = d.iter().into_iter();

        for tile in iterator{

            if col - x == d.width() as i16 {
                row += 1;
                col = x;
            }

            if col < 0 || col > w {
                col += 1;
                continue;
            };

            if row < 0 || row > h {
                col += 1;
                continue;
            };

            self.goto(col as u16, row as u16)
                .put(super::character_map::map(tile));
            col += 1;
        }

        self
    }
}
