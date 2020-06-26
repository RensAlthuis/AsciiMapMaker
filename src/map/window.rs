use super::Map;
use crate::graphics::Tile;

use std::rc::Rc;

pub struct WindowIterator<'a>{
    reference : &'a Map,
    window : &'a Window,
    index : usize,
    map_index : usize,
}

impl<'a> Iterator for WindowIterator<'a> {
    type Item = (usize, usize, Tile);

    fn next(&mut self) -> Option<Self::Item>{
        let tiles = self.reference.tiles.borrow();

        let width = self.window.width;
        let height  = self.window.height;

        let x = self.index % width;
        let y = self.index / width;


        if self.index < width*height {
            self.index +=1;

            let t = {
                let map_x = x as isize + self.window.x as isize;
                let map_y = y as isize + self.window.y as isize;

                if map_x < 0 || map_y < 0 || map_x >= self.reference.width as isize || map_y > self.reference.height as isize {
                    0
                }else {
                    let map_x = map_x as usize;
                    let map_y = map_y as usize;

                    *tiles.get(map_y*self.reference.width + map_x).unwrap_or(&0)
                }
            };

            Some((x, y, Tile::Tile(t)))
        }else{
            None
        }
    }

}

pub struct Window{
    map :  Rc<Map>,
    x : isize,
    y : isize,
    pub width: usize,
    pub height: usize,
}

impl Window{
    pub fn new(map: &Rc<Map>, x: isize, y : isize, width : usize, height : usize) -> Self {
        let mapref = Rc::clone(map);

        Window {
            map:mapref,
            x, y,
            width, height,
        }

    }

    pub fn set(&self, x : isize, y : isize, tile : usize) {
        let x = x + self.x;
        if x < 0 { return; }
        let y = y + self.y;
        if y < 0 { return; }

        self.map.set(x as usize, y as usize, tile);
    }

    pub fn scroll(&mut self, x : isize, y : isize){
        self.x += x;
        self.y += y;
    }
}


impl<'a> crate::graphics::Drawable<'a> for Window{

    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Tile)> + 'a>{
        let iter = WindowIterator{
            reference : &self.map,
            window : self,
            index : 0,
            map_index : 0,
        };
        Box::new(iter)
    }

    fn width(&self) -> usize {
        self.width
    }

}
