#[cfg(test)]
mod tests;

pub mod basic;

pub mod pmap;

pub mod imgbuff;

pub struct RGB(pub u8, pub u8, pub u8);

#[derive(Debug, PartialEq)]
pub enum Colourtype {
    Grayscale,
    GrayAlpha,
    Colour,
    ColourAlpha,
}
