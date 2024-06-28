use std::{u16, usize};

#[cfg(test)]
mod tests;

pub mod basic;

pub mod pmap;

pub struct RGB(pub u8, pub u8, pub u8);

#[derive(Debug, PartialEq)]
enum Colourtype {
    Grayscale,
    GrayAlpha,
    Colour,
    ColourAlpha,
}

#[derive(Debug, PartialEq)]
pub struct ImgBuffer {
    size: (u32, u32),
    ctype: Colourtype,
    body: Vec<u8>,
}
impl ImgBuffer {
    pub fn tograyscale(mut self) -> ImgBuffer {
        match self.ctype {
            Colourtype::Grayscale => self,
            Colourtype::GrayAlpha => self,
            Colourtype::Colour => {
                let mut newbody: Vec<u8> = vec![];
                let mut sumvar: u16 = 0;
                for (i, b) in self.body.iter().enumerate() {
                    sumvar += *b as u16;
                    if i % 3 == 2 {
                        newbody.push((sumvar / 3) as u8);
                    }
                }
                self.body = newbody;
                self.ctype = Colourtype::Grayscale;
                self
            }
            Colourtype::ColourAlpha => {
                let mut newbody: Vec<u8> = vec![];
                let mut sumvar: u16 = 0;
                for (i, b) in self.body.iter().enumerate() {
                    if i % 4 == 3 {
                        newbody.push((sumvar / 3) as u8);
                        sumvar = 0;
                    } else {
                        sumvar += *b as u16;
                    }
                }
                self.body = newbody;
                self.ctype = Colourtype::GrayAlpha;
                self
            }
        }
    }
    pub fn tocolour(mut self) -> ImgBuffer {
        self
    }
}
