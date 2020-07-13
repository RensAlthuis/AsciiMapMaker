use crate::graphics::{Drawable, Square, Style, StyleType};
use crate::graphics::Token;

pub struct MenuIterator {

}

pub struct Menu {
    options : Vec<(bool, String)>,
    selection : usize,
}

impl Menu {
    pub fn new(options: Vec<&str>) -> Self{
        Self{
            options : options.into_iter().map(|f| (false, String::from(f))).collect(),
            selection: 0
        }
    }

    pub fn up (&mut self) { if self.selection > 0 {self.selection -= 1;}}
    pub fn down(&mut self) { if self.selection < self.options.len()-1 { self.selection += 1}}
    pub fn toggle(&mut self) {
        if let Some(entry) = self.options.get_mut(self.selection){
            entry.0 = !entry.0;
        }
    }

    pub fn height(&self) -> usize{
        self.options.len() + 4
    }
}

impl<'a> Drawable<'a> for Menu {

    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a > {

        let sqr = Square::new(Token::Character('*'), self.width(), self.height());

        let strings = self.options.iter().enumerate().flat_map(
            move | (i, (b, s)) | {
                let drawable = if i != self.selection{
                    Style::new(Box::new(s.as_str()), StyleType::None)
                }else{
                    Style::new(Box::new(s.as_str()), StyleType::Underline)
                };

                let drawable = if *b {
                    Style::chain(drawable, StyleType::Colour{r:255,g:0,b:0})
                }else{
                    drawable
                };

                let drawable = Box::new(drawable).into_iter().map(
                    move |(x, y, c)| {
                        (x+2, 2+y+ i, c)
                    }
                );

                drawable.into_iter()
            }
        );

        let iter = Drawable::into_iter(Box::new(sqr)).chain(strings);
        Box::new(iter)
    }

    fn into_iter(self : Box<Self>) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a > {

        let sqr = Square::new(Token::Character('*'), self.width(), self.height());

        let selection = self.selection;
        let strings = self.options.into_iter().enumerate().flat_map(
            move | (i, (b, s)) | {
                let drawable = if i != selection{
                    Style::new(Box::new(s), StyleType::None)
                }else{
                    Style::new(Box::new(s), StyleType::Underline)
                };

                let drawable = if b {
                    Style::chain(drawable, StyleType::Colour{r:255,g:0,b:0})
                }else{
                    drawable
                };

                let drawable = Style::into_iter(Box::new(drawable)).map(
                    move |(x, y, c)| {
                        (x+2, 2+y+ i, c)
                    }
                );

                Box::new(drawable).into_iter()
            }
        );

        let iter = Drawable::into_iter(Box::new(sqr)).chain(strings);
        Box::new(iter)
    }

    fn width(&self) -> usize {
        (self.options.iter().map(|s| s.1.len()).max().unwrap()+4) as usize
    }

}