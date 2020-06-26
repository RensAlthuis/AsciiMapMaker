use super::Drawable;
use super::Tile;

pub struct Square{
    width: usize,
    height: usize,
    border : Tile,
    fill : Tile,
}

impl Square{
    pub fn filled(border : Tile, fill : Tile, width : usize, height : usize) -> Self {
        Square{
            width,
            height,
            border,
            fill
        }
    }

    pub fn new(border: Tile, width : usize, height : usize) -> Self {
        Square{
            width,
            height,
            border,
            fill : Tile::Empty,
        }
    }
}

impl<'a> Drawable<'a> for Square {

    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Tile)> + 'a >
    {
        let mut acc = Vec::with_capacity(self.width * self.height);

        for x in 0..self.width{
            acc.push((x, 0, self.border));
            acc.push((x, self.height-1, self.border));
        }

        for y in 1..(self.height-1) {
            acc.push((0, y, self.border));
            acc.push((self.width-1, y, self.border));
        }

        for x in 1..(self.width - 1) {
            for y in 1..(self.height - 1) {
                acc.push((x, y, self.fill));
            }
        }

        Box::new(acc.into_iter())
    }

    fn width(&self) -> usize {
        self.width
    }

}

impl IntoIterator for Square{
    type Item = (usize, usize, Tile);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut acc = Vec::with_capacity(self.width * self.height);

        for x in 0..self.width{
            acc.push((x, 0, self.border));
            acc.push((x, self.height-1, self.border));
        }
        for y in 1..(self.height-1) {
            acc.push((0, y, self.border));
            acc.push((self.width-1, y, self.border));
        }

        for x in 1..(self.width - 1) {
            for y in 1..(self.height - 1) {
                acc.push((x, y, self.fill));
            }
        }

        acc.into_iter()
    }

}