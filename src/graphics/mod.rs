mod character_map;
pub mod square;
pub mod terminal;

pub use square::Square;

#[derive(Clone, Copy)]
pub enum Tile{
    Character(char),
    Tile(usize),
    Style (Style),
    Empty
}

#[derive(Clone, Copy)]
pub enum Style {
    Italic,
    Bold,
    Underline,
    NoUnderline
}

pub enum Event {
    Key(char),
    MouseDown(usize, usize, usize),
    MouseDrag(usize, usize),
    MouseUp(usize, usize)
}

pub trait Drawable<'a> {
    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Tile)> + 'a>;
    fn width(&self) -> usize;
}

pub trait Graphics {
    fn run<F>(&mut self, f : &mut F) where F : FnMut(&mut Self, Event) -> bool;
    fn draw<'a>(&mut self, map: &'a impl Drawable<'a>, x: isize, y: isize) -> &mut Self;
    fn size(&self) -> (usize, usize);
}

impl<'a> Drawable<'a> for String{
    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Tile)> + 'a> {
        Box::new(self.chars().enumerate().map(
            | c | (c.0, 0, Tile::Character(c.1))
        ))
    }

    fn width(&self) -> usize {
        self.len()
    }

}

impl<'a> Drawable<'a> for str {
    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Tile)> + 'a> {
        Box::new(self.chars().enumerate().map(
            | c | (c.0, 0, Tile::Character(c.1))
        ))
    }

    fn width(&self) -> usize {
        self.len()
    }
}