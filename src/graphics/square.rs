use super::Drawable;
use super::token::Token;

pub struct Square{
    width: usize,
    height: usize,
    border : Token,
    fill : Token,
}

impl Square{
    pub fn filled(border : Token, fill : Token, width : usize, height : usize) -> Self {
        Square{
            width,
            height,
            border,
            fill
        }
    }

    pub fn new(border: Token, width : usize, height : usize) -> Self {
        Square{
            width,
            height,
            border,
            fill : Token::Empty,
        }
    }
}

impl<'a> Drawable<'a> for Square {

    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a >
    {
        let mut acc = Vec::with_capacity(self.width * self.height);

        for x in 0..self.width{
            acc.push((x, 0, self.border.clone()));
            acc.push((x, self.height-1, self.border.clone()));
        }

        for y in 1..(self.height-1) {
            acc.push((0, y, self.border.clone()));
            acc.push((self.width-1, y, self.border.clone()));
        }

        for x in 1..(self.width - 1) {
            for y in 1..(self.height - 1) {
                acc.push((x, y, self.fill.clone()));
            }
        }

        Box::new(acc.into_iter())
    }

    fn into_iter(self : Box<Self>) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a >
    {
        let mut acc = Vec::with_capacity(self.width * self.height);

        for x in 0..self.width{
            acc.push((x, 0, self.border.clone()));
            acc.push((x, self.height-1, self.border.clone()));
        }

        for y in 1..(self.height-1) {
            acc.push((0, y, self.border.clone()));
            acc.push((self.width-1, y, self.border.clone()));
        }

        for x in 1..(self.width - 1) {
            for y in 1..(self.height - 1) {
                acc.push((x, y, self.fill.clone()));
            }
        }

        Box::new(acc.into_iter())
    }

    fn width(&self) -> usize {
        self.width
    }

}