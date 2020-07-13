use crate::graphics::style::StyleType;
use byteorder::{ByteOrder, LittleEndian};
use utf8_decode::decode;

fn read_four_bytes(stream: &mut impl Iterator<Item = u8>) -> Option<[u8; 4]>{
    let mut buf = [0u8; 4];
    for i in 0..4{
        if let Some(byte) =
            stream.next()
                  .and_then(|byte|{Some(byte)})
        {
            buf[i] = byte;
        } else {
            return None;
        }
    };

    Some(buf)
}

#[derive(Clone)]
pub enum Token{
    Character(char),
    Tile(usize),
    StyleType(StyleType),
    Style(StyleType, Box<Token>),
    Empty
}

impl Token{
    pub fn parse(mut stream : &mut impl Iterator<Item = u8>) -> Option<Token> {

        if let Some(b) = stream.next(){
            match b {
                0 => {
                    Some(Token::Empty)
                },
                1 => {
                    decode(stream).and_then(|res| {
                        res.ok()
                    }).and_then(|c|{
                        Some(Token::Character(c))
                    })
                },
                2 => {
                    read_four_bytes(&mut stream).map(|buf|{
                        LittleEndian::read_u32(&buf) as usize
                    }).map(|u| {
                        Token::Tile(u)
                    })
                }
                3 => {
                    None //TODO
                    // let buf = read_four_bytes(&mut stream);
                    // let style : &[u8; 4]= &[buf[1], buf[2], buf[3], buf[4]];
                    // Token::Style(style.into())
                },
                4 => {
                    None
                }
                _ => None
            }
        }else{
            None
        }
    }
}

impl From<Token> for std::vec::Vec<u8>{
    fn from(token: Token) -> Self {
        match token {
            Token::Empty => vec![0],
            Token::Character(c) => {
                let mut buf = vec![1, 0, 0, 0, 0];
                c.encode_utf8(&mut buf[1..5]);
                buf
            },
            Token::Tile(i) => {
                let mut buf = vec![2, 0, 0, 0, 0];
                LittleEndian::write_u32(&mut buf[1..5], i as u32);
                buf
            },
            Token::StyleType(_s) => {
                vec![] //TODO
            },
            Token::Style(_s, _t) => {
                vec![] //TODO
                // let r : &[u8] = transmute(t);
                // res[0] = 3;
                // for i in 1..5{
                //     res[i] = r[i-1];
                // }
            }
        }
    }
}