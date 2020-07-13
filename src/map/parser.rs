use super::Token;
use std::io;

pub fn from_bytes(stream: impl Iterator<Item = Result<u8, io::Error>>) -> Vec<Token>{
    let mut v = Vec::new();
    let mut stream = stream.map(|e| e.unwrap());
    loop{
        if let Some(token) = Token::parse(&mut stream){
            v.push(token);
        }else{
            break;
        }
    }
    v
}