mod character_map;
pub mod square;
pub mod terminal;

pub use square::Square;

pub trait Drawable<'a> {

    fn iter(&'a self) -> Box<dyn Iterator<Item = u32> + 'a>;
    fn width(&self) -> u16;
}

pub trait Graphics {
    fn run<F>(&mut self, f : F) where F : Fn(&mut Self, Event) -> bool;
    fn draw<'a>(&mut self, map: &'a impl Drawable<'a>, x: i16, y: i16) -> &mut Self;
}

pub enum Event {
    Key(char),
    MouseDown(u16, u16, u16),
    MouseDrag(u16, u16),
    MouseUp(u16, u16)
}
