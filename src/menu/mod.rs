use crate::graphics::{Drawable, Square};

pub struct Menu {
    options : Vec<String>,
}

impl Menu {
    pub fn new() -> Self{
        let options = vec!["hello", "cheese", "Something else", "another thing"];

        Self{
            options : options.into_iter().map(|f| String::from(f)).collect()
        }
    }
}

impl<'a> Drawable<'a> for Menu {

    fn iter<>(&'a self) -> Box<dyn Iterator<Item = u32> + 'a > {

        let sqr = Square::new(1, self.width(), (self.options.len()+4) as u16);
        let iter = sqr.into_iter();
        Box::new(iter)
    }

    fn width(&self) -> u16 {
        self.options.iter().map(|s| s.len()).max().unwrap() as u16
    }

}