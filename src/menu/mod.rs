use crate::graphics::{Drawable, Square, Tile, Style};

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

    pub fn height(&self) -> usize{
        self.options.len() + 4
    }
}

//TODO: Find a better way to do string styling
impl<'a> Drawable<'a> for Menu {

    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Tile)> + 'a > {

        let sqr = Square::new(Tile::Character('*'), self.width(), self.height());

        let strings = self.options.iter().enumerate().flat_map(
            move | (i, (b, s)) | {
                Drawable::iter(s).map(
                    move |(x, y, c)| {
                        (x+2, 2+y+ i, c)
                    }
                )
            }
        ).fold(Vec::new(),
            |mut acc, (x,y,c)|{
                if x == 2 && y == self.selection+2{
                    acc.push((0, y, Tile::Style(Style::Underline)));
                }

                acc.push((x, y, c));

                if x == self.options[y-2].1.len()+1 && y == self.selection+2 {
                    acc.push((0, y, Tile::Style(Style::NoUnderline)));
                }

                acc
            }
        );


        let iter = sqr.into_iter().chain(strings);
        Box::new(iter)
    }

    fn width(&self) -> usize {
        (self.options.iter().map(|s| s.1.len()).max().unwrap()+4) as usize
    }

}