use super::token::Token;
use crate::graphics::Drawable;

#[derive(Copy, Clone)]
pub enum StyleType {
    Underline, NoUnderline,
    Italic, NoItalic,
    Bold, NoBold,
    Colour{r: u8, g: u8, b: u8}, ResetColour,
    None
}

impl StyleType {
    pub fn invert(&self) -> StyleType {
        match self {
            StyleType::Underline => StyleType::NoUnderline,
            StyleType::Italic => StyleType::NoItalic,
            StyleType::Bold => StyleType::NoBold,
            StyleType::NoUnderline => StyleType::Underline,
            StyleType::NoItalic => StyleType::Italic,
            StyleType::NoBold => StyleType::Bold,
            StyleType::None => StyleType::None,
            StyleType::Colour{r:_, g:_, b:_} => StyleType::ResetColour,
            StyleType::ResetColour => StyleType::None,
        }
    }
}

pub struct Style<'a> {
    inner : Box<dyn Drawable<'a> + 'a>,
    style_type : StyleType,
}

impl<'a> Style<'a>{
    pub fn new(drawable: Box<dyn Drawable<'a> +'a>, style_type: StyleType)-> Self{
        Style{
            inner : drawable,
            style_type
        }
    }

    pub fn chain(style : Style<'a>, style_type : StyleType)-> Self {
        Style{
            inner : Box::new(style),
            style_type
        }
    }
}

impl<'a> Drawable<'a> for Style<'a>{
    fn iter(&'a self) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a> {
        let start = Some((0, 0, Token::StyleType(self.style_type))).into_iter();
        let end = Some((0, 0, Token::StyleType(self.style_type.invert()))).into_iter();
        let iter = start.chain(self.inner.iter().chain(end));
        Box::new(iter)
    }

    fn into_iter(self : Box<Self>) -> Box<dyn Iterator<Item = (usize, usize, Token)> + 'a>{
        let start = Some((0, 0, Token::StyleType(self.style_type))).into_iter();
        let end = Some((0, 0, Token::StyleType(self.style_type.invert()))).into_iter();
        let iter = start.chain(self.inner.into_iter().chain(end));
        Box::new(iter)
    }

    fn width(&self) -> usize {
        self.inner.width()
    }
}