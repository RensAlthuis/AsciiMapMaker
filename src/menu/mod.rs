use crate::graphics::{Drawable, Square};

struct Menu {
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
    type Target = std::vec::IntoIter<u32>;

    fn iter(&'a self) -> Box<Self::Target> {
        let w = self.options.iter().map(|s| s.len()).max().unwrap();

        let sqr = Square::new(1, w as u16, (self.options.len()+4) as u16);
        let iter = sqr.into_iter();
        Box::new(iter)

    }
    fn width(&self) -> u16 {
        todo!()
    }

}