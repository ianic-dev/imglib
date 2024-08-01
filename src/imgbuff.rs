use std::usize;

use crate::Colourtype;

pub enum Pixel {
    Grayscale(&mut u8),
    GrayAlpha([&mut u8; 2]),
    Colour([&mut u8; 3]),
    ColourAlpha([&mut u8; 4]),
}

#[derive(Debug, PartialEq)]
pub struct ImgBuffer {
    pub size: (u32, u32),
    pub ctype: Colourtype,
    pub body: Vec<u8>,
}
impl ImgBuffer {
    pub fn new(size: (u32, u32), ctype: Colourtype, body: Vec<u8>) -> ImgBuffer {
        let npix = size.0 as usize * size.1 as usize;
        match ctype {
            Colourtype::Grayscale => assert_eq!(body.len(), npix),
            Colourtype::GrayAlpha => assert_eq!(body.len(), npix * 2),
            Colourtype::Colour => assert_eq!(body.len(), npix * 3),
            Colourtype::ColourAlpha => assert_eq!(body.len(), npix * 4),
        }
        ImgBuffer { size, ctype, body }
    }
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
                        sumvar = 0;
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
        match self.ctype {
            Colourtype::Grayscale => {
                let mut newbody: Vec<u8> = vec![];
                for b in self.body {
                    newbody.push(b);
                    newbody.push(b);
                    newbody.push(b);
                }
                self.body = newbody;
                self.ctype = Colourtype::Colour;
                self
            }
            Colourtype::GrayAlpha => {
                let mut newbody: Vec<u8> = vec![];
                let mut c = 0;
                for b in self.body {
                    if c & 1 == 0 {
                        newbody.push(b);
                        newbody.push(b);
                        newbody.push(b);
                    } else {
                        newbody.push(b);
                    }
                    c += 1;
                }
                self.body = newbody;
                self.ctype = Colourtype::Colour;
                self
            }
            Colourtype::Colour => self,
            Colourtype::ColourAlpha => self,
        }
    }
    pub fn rmalpha(mut self) -> ImgBuffer {
        match self.ctype {
            Colourtype::Grayscale => {}
            Colourtype::GrayAlpha => {
                let mut newbody: Vec<u8> = vec![];
                for (i, b) in self.body.iter().enumerate() {
                    if i & 1 == 0 {
                        newbody.push(*b);
                    }
                }
                self.body = newbody;
                self.ctype = Colourtype::Grayscale;
            }
            Colourtype::Colour => {}
            Colourtype::ColourAlpha => {
                let mut newbody: Vec<u8> = vec![];
                for (i, b) in self.body.iter().enumerate() {
                    if i & 4 != 3 {
                        newbody.push(*b);
                    }
                }
                self.body = newbody;
                self.ctype = Colourtype::Colour;
            }
        };
        self
    }
    pub fn addalpha(mut self) -> ImgBuffer {
        match self.ctype {
            Colourtype::Grayscale => {
                let mut newbody: Vec<u8> = vec![];
                for b in self.body.iter() {
                    newbody.push(*b);
                    newbody.push(0xff);
                }
                self.body = newbody;
                self.ctype = Colourtype::GrayAlpha;
            }
            Colourtype::GrayAlpha => {}
            Colourtype::Colour => {
                let mut newbody: Vec<u8> = vec![];
                for (i, b) in self.body.iter().enumerate() {
                    newbody.push(*b);
                    if i % 3 == 2 {
                        newbody.push(0xff);
                    }
                }
            }
            Colourtype::ColourAlpha => {}
        }
        self
    }
    pub fn getpixelref(&mut self, mut x: u32, y: u32) -> Pixel {
        x += y * self.size.0;
        let x = x as usize;
        match self.ctype {
            Colourtype::Grayscale => Pixel::Grayscale(&mut self.body[x]),
            Colourtype::GrayAlpha => Pixel::GrayAlpha(&mut self.body[2 * x..2 * x + 2]),
            Colourtype::Colour => Pixel::Colour(&mut self.body[3 * x..3 * x + 3]),
            Colourtype::ColourAlpha => Pixel::ColourAlpha(&mut self.body[4 * x..4 * x + 4]),
        }
    }
    pub fn getbodyref(&mut self) -> Vec<Vec<Pixel>> {
        let mut bodyrefs: Vec<_> = vec![];
        for row_ind in 0..(self.size.1 as usize) {
            let mut rowrefs: Vec<Pixel> = vec![];
            for col_ind in 0..(self.size.0 as usize) {
                let absind = col_ind + row_ind * self.size.0 as usize;
                rowrefs.push(match self.ctype {
                    Colourtype::Grayscale => Pixel::Grayscale(&mut self.body[absind]),
                    Colourtype::GrayAlpha => {
                        Pixel::GrayAlpha(&mut self.body[2 * absind..2 * absind + 2])
                    }
                    Colourtype::Colour => Pixel::Colour(&mut self.body[3 * absind..3 * absind + 3]),
                    Colourtype::ColourAlpha => {
                        Pixel::ColourAlpha(&mut self.body[4 * absind..4 * absind + 4])
                    }
                })
            }
            bodyrefs.push(rowrefs);
        }
        bodyrefs
    }
}
