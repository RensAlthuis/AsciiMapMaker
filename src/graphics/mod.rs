mod square;
pub mod terminal;
mod style;
mod token;

pub use square::Square;
pub use style::{Style, StyleType};
pub use token::Token;

pub enum Event {
    Key(char),
    MouseDown(usize, usize, usize),
    MouseDrag(usize, usize, usize),
    MouseUp(usize, usize)
}

pub trait Drawable<'a> {
    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a>;
    fn into_iter(self : Box<Self>) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a>;
    fn width(&self) -> usize;
}

pub trait Graphics {
    fn run<F>(&mut self, keymap : &mut crate::KeyMap<F>, context: &mut F, f : fn(&mut Self, &mut crate::KeyMap<F>, Event, &mut F) -> bool);
    fn draw<'a>(&mut self, map: &'a impl Drawable<'a>, x: isize, y: isize) -> &mut Self;
    fn size(&self) -> (usize, usize);
}

impl<'a> Drawable<'a> for String{
    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a> {
        Box::new(self.chars().enumerate().map(
            | c | (c.0, 0, Token::Character(c.1))
        ))
    }

    fn into_iter(self : Box<Self>) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a> {
        let vec : Vec<char> = self.chars().collect();
        Box::new(vec.into_iter().enumerate().map(
            move | c | (c.0, 0, Token::Character(c.1))
        ))
    }

    fn width(&self) -> usize {
        self.len()
    }

}

impl<'a> Drawable<'a> for &'a str {
    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a> {
        Box::new(self.chars().enumerate().map(
            | c | (c.0, 0, Token::Character(c.1))
        ))
    }

    fn into_iter(self : Box<Self>) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a> {
        Box::new(self.chars().enumerate().map(
            | c | (c.0, 0, Token::Character(c.1))
        ))
    }

    fn width(&self) -> usize {
        self.len()
    }
}

impl<'a> Drawable<'a> for Box<str> {
    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a> {
        Box::new(self.chars().enumerate().map(
            | c | (c.0, 0, Token::Character(c.1))
        ))
    }

    fn into_iter(self : Box<Self>) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a> {
        let vec : Vec<char> = self.chars().collect();
        Box::new(vec.into_iter().enumerate().map(
            | c | (c.0, 0, Token::Character(c.1))
        ))
    }

    fn width(&self) -> usize {
        self.len()
    }
}

impl<'a> Drawable<'a> for str {
    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a> {
        Box::new(self.chars().enumerate().map(
            | c | (c.0, 0, Token::Character(c.1))
        ))
    }

    fn into_iter(self : Box<Self>) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a> {
        let vec : Vec<char> = self.chars().collect();
        Box::new(vec.into_iter().enumerate().map(
            | c | (c.0, 0, Token::Character(c.1))
        ))
    }

    fn width(&self) -> usize {
        self.len()
    }
}