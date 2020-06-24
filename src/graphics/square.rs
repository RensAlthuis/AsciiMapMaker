use super::Drawable;

pub struct Square{
    width: u16,
    height: u16,
    border : u32,
    fill : u32,
}

impl Square{
    pub fn filled(border : u32, fill : u32, width : u16, height : u16) -> Self {
        Square{
            width,
            height,
            border,
            fill
        }
    }

    pub fn new(border: u32, width : u16, height : u16) -> Self {
        Square{
            width,
            height,
            border,
            fill : 0,
        }
    }
}

impl<'a> Drawable<'a> for Square {
    type Target = std::vec::IntoIter<u32>;

    fn iter(&'a self) -> Box<Self::Target>
    {
        let mut acc = Vec::with_capacity(self.width as usize * self.height as usize);

        for _ in 0..self.width{
            acc.push(self.border)
        }

        for _ in 0..(self.height - 2) {
            acc.push(self.border);
            for _ in 0..(self.width-2) {
                acc.push(self.fill);
            }
            acc.push(self.border);
        }

        for _ in 0..self.width{
            acc.push(self.border)
        }

        let iter = acc.into_iter();

        Box::new(iter)
    }

    fn width(&self) -> u16 {
        self.width as u16
    }

}

impl IntoIterator for Square{
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut acc = Vec::with_capacity(self.width as usize * self.height as usize);

        for _ in 0..self.width{
            acc.push(self.border)
        }

        for _ in 0..(self.height - 2) {
            acc.push(self.border);
            for _ in 0..(self.width-2) {
                acc.push(self.fill);
            }
            acc.push(self.border);
        }

        for _ in 0..self.width{
            acc.push(self.border)
        }

        acc.into_iter()
    }

}