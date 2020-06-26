
pub fn map(tile : usize) -> char
{
    match tile{
            0 => ' ',
            1 => '.',
            2 => 'X',
            _ => ' ',
        }
}