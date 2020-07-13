pub mod window;
pub mod parser;

pub use window::Window;
use crate::graphics::Token;

use byteorder::{ByteOrder, LittleEndian};
use byteorder::ReadBytesExt;
use std::io::prelude::*;
use std::cell::RefCell;
use std::{ops::Deref, rc::Rc};

pub struct Map{
    pub tiles : RefCell<Vec<Token>>,
    pub width : usize,
    pub height : usize,
}

impl Map{
    pub fn new (width : usize, height : usize) -> Self {
        let mut tiles = Vec::with_capacity(width*height);
        for _ in 0..(width*height){
            tiles.push(Token::Tile(1));
        }

        Map {
            tiles : RefCell::new(tiles),
            width,
            height
        }
    }

    pub fn set(&self, x : usize, y : usize, token : Token) {
        let mut tiles = self.tiles.borrow_mut();
        let t = tiles.get_mut(y*self.width + x).unwrap();
        *t = token;
    }

    pub fn from_file(path : &str) -> Self{

        let mut file = std::fs::File::open(path).unwrap();
        let width = file.read_u32::<LittleEndian>().unwrap() as usize;
        let height = file.read_u32::<LittleEndian>().unwrap() as usize;
        let nbytes = file.metadata().unwrap().len();
        let bytes = file.take((nbytes) as u64).bytes();
        let tiles = parser::from_bytes(bytes);

        Map {
            tiles : RefCell::new(tiles),
            width, height
        }
    }

    pub fn to_file(&self, path : &str){
        let mut file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
        let mut buf = [0u8; 4];

        LittleEndian::write_u32(&mut buf, self.width as u32);
        file.write(&buf).unwrap();

        LittleEndian::write_u32(&mut buf, self.height as u32);
        file.write(&buf).unwrap();

        for e in self.tiles.borrow().iter(){
            let bytes : Vec<u8> = (e.clone()).into();
            file.write(&bytes).unwrap();
        }
    }
}

#[derive(Clone)]
pub struct MapRef{
    pub inner : Rc<Map>
}

impl MapRef{
    pub fn new (map: Map) -> Self {
        MapRef{
            inner : Rc::new(map)
        }
    }
}

impl Deref for MapRef{
    type Target = Map;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }

}