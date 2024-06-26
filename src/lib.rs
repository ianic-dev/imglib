#[cfg(test)]
mod tests;

pub mod basic;

pub mod pmap;

pub struct RGB(u8, u8, u8);

#[derive(Debug)]
enum Colourtype {
    Grayscale,
    GrayAlpha,
    Colour,
    ColourAlpha,
}

#[derive(Debug)]
pub struct ImgBuffer {
    size: (usize, usize),
    ctype: Colourtype,
    double: bool,
    buffer: Vec<u8>,
}
