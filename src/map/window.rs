use super::MapRef;
use super::Token;

pub struct WindowIterator{
    reference : MapRef,
    index : usize,
    map_index : usize,
    window_x : isize,
    window_y : isize,
    window_width : usize,
    window_height : usize,
}

impl<'a> Iterator for WindowIterator{
    type Item = (usize, usize, Token);

    fn next(&mut self) -> Option<Self::Item>{
        let tiles = self.reference.tiles.borrow();

        let width = self.window_width;
        let height  = self.window_height;

        let x = self.index % width;
        let y = self.index / width;


        if self.index < width*height {
            self.index +=1;

            let t = {
                let map_x = (self.map_index % width) as isize + self.window_x as isize;
                let map_y = (self.map_index / width) as isize + self.window_y as isize;

                if map_x < 0 || map_y < 0 || map_x >= self.reference.width as isize || map_y > self.reference.height as isize {
                    Token::Tile(0)
                }else {
                    let map_x = map_x as usize;
                    let map_y = map_y as usize;

                    tiles.get(map_y*self.reference.width + map_x).unwrap_or(&Token::Character(' ')).clone()
                }
            };
            match t {
                Token::Tile(_) => self.map_index += 1,
                Token::Style(_, _) => self.map_index += 1,
                _ => ()
            }

            Some((x, y, t))
        }else{
            None
        }
    }

}

pub struct Window{
    map :  MapRef,
    x : isize,
    y : isize,
    pub width: usize,
    pub height: usize,
}

impl Window{
    pub fn new(map: MapRef, x: isize, y : isize, width : usize, height : usize) -> Self {

        Window {
            map,
            x, y,
            width, height,
        }

    }

    pub fn set(&self, x : isize, y : isize, token : Token) {
        let x = x + self.x;
        if x < 0 { return; }
        let y = y + self.y;
        if y < 0 { return; }

        self.map.set(x as usize, y as usize, token);
    }

    pub fn scroll(&mut self, x : isize, y : isize){
        self.x += x;
        self.y += y;
    }
}


impl<'a> crate::graphics::Drawable<'a> for Window{

    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a>{
        let iter = WindowIterator{
            reference : self.map.clone(),
            index : 0,
            map_index : 0,
            window_x : self.x,
            window_y : self.y,
            window_width : self.width,
            window_height : self.height,
        };
        Box::new(iter)
    }

    fn into_iter(self : Box<Self>) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a>{
        let iter = WindowIterator{
            reference : self.map.clone(),
            index : 0,
            map_index : 0,
            window_x : self.x,
            window_y : self.y,
            window_width : self.width,
            window_height : self.height,
        };
        Box::new(iter)
    }

    fn width(&self) -> usize {
        self.width
    }

}
