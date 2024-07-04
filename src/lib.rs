#[cfg(test)]
mod tests;

pub mod basic;

pub mod pmap;

pub mod imgbuff;

#[derive(Debug, PartialEq)]
pub enum Colourtype {
    Grayscale,
    GrayAlpha,
    Colour,
    ColourAlpha,
}
