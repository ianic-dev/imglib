use crate::{basic, imgbuff::ImgBuffer, Colourtype};

mod makebody;
mod readbody;

#[derive(Debug, PartialEq)]
pub enum PNMtype {
    PBM,
    PGM,
    PPM,
}

#[derive(Debug, PartialEq)]
pub struct PNMap {
    ident: PNMtype,
    size: (u32, u32),
    pub body: Vec<u8>,
}
impl PNMap {
    pub fn new(ident: PNMtype, size: (u32, u32), body: Vec<u8>) -> PNMap {
        match ident {
            PNMtype::PBM => assert_eq!(body.len(), size.0 as usize * size.1 as usize),
            PNMtype::PGM => assert_eq!(body.len(), size.0 as usize * size.1 as usize),
            PNMtype::PPM => assert_eq!(body.len(), 3 * size.0 as usize * size.1 as usize),
        }
        PNMap { ident, size, body }
    }
}

pub fn makepbm(anyimage: ImgBuffer, treshold: Option<u32>) -> PNMap {
    let treshold = treshold.unwrap_or(128);
    let image = anyimage.rmalpha().tograyscale();
    assert_eq!(image.ctype, Colourtype::Grayscale);
    let ident = PNMtype::PBM;
    let mut body: Vec<u8> = vec![];
    for b in image.body {
        if b as u32 >= treshold {
            body.push(0);
        } else {
            body.push(1);
        }
    }
    PNMap::new(ident, image.size, body)
}

pub fn makepgm(anyimage: ImgBuffer) -> PNMap {
    let image = anyimage.rmalpha().tograyscale();
    assert_eq!(image.ctype, Colourtype::Grayscale);
    let ident = PNMtype::PGM;
    PNMap::new(ident, image.size, image.body)
}

pub fn makeppm(anyimage: ImgBuffer) -> PNMap {
    let image = anyimage.rmalpha();
    let ident = PNMtype::PPM;
    match image.ctype {
        Colourtype::Grayscale => todo!(),
        Colourtype::GrayAlpha => panic!("somehow, the bug appeared"),
        Colourtype::Colour => PNMap::new(ident, image.size, image.body),
        Colourtype::ColourAlpha => panic!("this isn't supposed to happen"),
    }
}

pub fn writepnm(image: PNMap, plain: bool) -> Vec<u8> {
    let mut file: Vec<u8> = vec![80];
    match (&image.ident, plain) {
        (PNMtype::PBM, true) => file.push(49),
        (PNMtype::PGM, true) => file.push(50),
        (PNMtype::PPM, true) => file.push(51),
        (PNMtype::PBM, false) => file.push(52),
        (PNMtype::PGM, false) => file.push(53),
        (PNMtype::PPM, false) => file.push(54),
    };
    for b in String::from(
        "\n# file written by ianic-dev's imglib, built referencing the netpbm documentation\n",
    )
    .as_bytes()
    {
        file.push(*b);
    }
    for b in basic::makeascii(image.size.0) {
        file.push(b);
    }
    file.push(32);
    for b in basic::makeascii(image.size.1) {
        file.push(b);
    }
    file.push(10);
    if image.ident != PNMtype::PBM {
        for b in basic::makeascii(255) {
            file.push(b);
        }
        file.push(10);
    }
    println!("{file:?}");
    let body = match (&image.ident, plain) {
        (PNMtype::PBM, true) => makebody::mkbodypbmplain(image),
        (PNMtype::PGM, true) => makebody::mkbodypgmplain(image),
        (PNMtype::PPM, true) => makebody::mkbodyppmplain(image),
        (PNMtype::PBM, false) => makebody::mkbodypbmraw(image),
        (PNMtype::PGM, false) => makebody::mkbodypgmraw(image),
        (PNMtype::PPM, false) => makebody::mkbodyppmraw(image),
    };
    println!("{body:?}");
    for b in body {
        file.push(b);
    }
    if !plain {
        file.push(10);
    }
    file
}

pub fn parsepnm(file: Vec<u8>) -> Result<ImgBuffer, &'static str> {
    //println!("{}{}", file[0], file[1]);
    if file[0] != 80 {
        println!("no P");
        return Err("this is not a valid netpbm format");
    }
    let ctype = match file[1] {
        49 | 50 | 52 | 53 => Colourtype::Grayscale,
        51 | 54 => Colourtype::Colour,
        55 => return Err("this library has no support for the PAM format"),
        _ => return Err("this is not a valid netpbm format"),
    };
    let hasmaxval: bool = !(file[1] == 49 || file[1] == 52);
    //println!("hasmaxval = {}", hasmaxval);
    let mut state: usize = 1;
    let mut width: u32 = 1;
    let mut height: u32 = 1;
    let mut marker: usize = 2;
    let mut backup: usize = 0;
    let mut cdepth: u32 = 256;
    'header: for (i, byte) in file.iter().enumerate() {
        if i > 1 {
            match (state, *byte) {
                (_, 35) => {
                    //println!("comment in state {}", state);
                    backup = marker;
                    marker = state;
                    state = 0;
                }
                (0, 10 | 13) => {
                    state = marker;
                    marker = backup;
                    //println!("return to state {} after comment", state);
                }
                (1, 48..=57) => {
                    //println!("entering state {}", marker);
                    state = marker;
                    marker = i;
                }
                (2, 9..=13 | 32) => {
                    width = basic::numfromascii(&file[marker..i]);
                    //println!("width found: is {}", width);
                    marker = 3;
                    state = 1;
                }
                (3, 9..=13 | 32) => {
                    height = basic::numfromascii(&file[marker..i]);
                    //println!("height found: is {}", height);
                    if hasmaxval {
                        marker = 4;
                        state = 1;
                    } else {
                        marker = i;
                        break 'header;
                    }
                }
                (4, 9..=13 | 32) => {
                    cdepth = basic::numfromascii(&file[marker..i]);
                    //println!("maxval found: is {}", cdepth);
                    marker = i;
                    break 'header;
                }
                (0 | 1 | 2 | 3 | 4, _) => {}
                (_, _) => {
                    println!("state {} byte {}", state, byte);
                    return Err("I have no idea how you managed to land in this invalid state");
                }
            }
        }
    }
    //println!("body starts at byte {} which is {}", marker, file[marker]);
    let mut body = match file[1] {
        49 => readbody::rdbodypbmplain(file, marker),
        50 => readbody::rdbodypgmplain(file, marker, cdepth),
        51 => readbody::rdbodyppmplain(file, marker, cdepth),
        52 => readbody::rdbodypbmraw(file, marker, (width, height)),
        53 => readbody::rdbodypgmraw(file, marker, (width, height), cdepth),
        54 => readbody::rdbodyppmraw(file, marker, (width, height), cdepth),
        _ => return Err("this is not a valid netpbm format"),
    };
    body = dbg!(body);
    println!("{}", body.len());

    Ok(ImgBuffer::new((width, height), ctype, body))
}
