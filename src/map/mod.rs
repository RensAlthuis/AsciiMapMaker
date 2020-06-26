pub mod window;
pub use window::Window;

use byteorder::{ByteOrder, LittleEndian};
use byteorder::ReadBytesExt;
use std::io::prelude::*;
use std::cell::RefCell;

pub struct Map{
    pub tiles : RefCell<Vec<usize>>,
    pub width : usize,
    pub height : usize,
}

impl Map{
    pub fn new (width : usize, height : usize) -> Self {
        let mut tiles = Vec::with_capacity(width*height);
        for _ in 0..(width*height){
            tiles.push(1);
        }

        Map{
            tiles : RefCell::new(tiles),
            width,
            height
        }
    }

    pub fn set(&self, x : usize, y : usize, tile : usize) {
        let mut tiles = self.tiles.borrow_mut();
        let t = tiles.get_mut(y*self.width + x).unwrap();
        *t = tile;
    }

    pub fn from_file(path : &str) -> Self{

        let mut file = std::fs::File::open(path).unwrap();
        let width = file.read_u32::<LittleEndian>().unwrap() as usize;
        let height = file.read_u32::<LittleEndian>().unwrap() as usize;
        let mut tiles : Vec<usize> = Vec::with_capacity(width*height);

        for _ in 0..width*height {
            tiles.push(file.read_u32::<LittleEndian>().unwrap() as usize);
        }

        Map{
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
            LittleEndian::write_u32(&mut buf, *e as u32);
            file.write(&buf).unwrap();
        }
    }
}