use super::Map;

use std::rc::Rc;
use std::cell::Ref;

pub struct WindowIterator<'a>{
    reference : Ref<'a, Vec<u32>>,
    window : &'a Window,
    index : usize,
}

impl<'a> Iterator for WindowIterator<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        let width = self.window.width;
        let height  = self.window.height;
        let x = self.window.x + (self.index % width);
        let y = self.window.y + (self.index / width);

        if self.index < width*height {
            let res = Some(self.reference[y*width + x]);
            self.index +=1;
            res
        }else{
            None
        }
    }

}

pub struct Window{
    map :  Rc<Map>,
    x : usize,
    y : usize,
    pub width: usize,
    pub height: usize,
}

impl Window{
    pub fn new(map: &Rc<Map>, x: usize, y : usize, width : usize, height : usize) -> Self {
        let mapref = Rc::clone(map);

        Window {
            map:mapref,
            x, y,
            width, height,
        }

    }

    pub fn set(&self, x : usize, y : usize, tile : u32) {
        self.map.set(x+self.x, y+self.y, tile);
    }
}


impl<'a> crate::graphics::Drawable<'a> for Window{

    fn iter(&'a self) -> Box<dyn Iterator<Item = u32> + 'a>{

        let tiles = self.map.tiles.borrow();
        Box::new(WindowIterator{
            reference : tiles,
            window : self,
            index : 0,
        })
    }

    fn width(&self) -> u16 {
        self.width as u16
    }

}
