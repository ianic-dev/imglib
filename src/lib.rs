#[cfg(test)]
mod tests;

pub mod basic;

pub mod pmap;

pub struct RGB(u8, u8, u8);

#[derive(Debug)]
enum Colourtype {
    Grayscale(u8),
    GrayAlpha(u8),
    Colour(u8),
    ColourAlpha(u8),
}

#[derive(Debug)]
pub struct ImgBuffer {
    dim: (u32, u32),
    ctype: Colourtype,
    buffer: Vec<u8>,
}
