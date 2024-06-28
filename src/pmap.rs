use crate::{basic, Colourtype, ImgBuffer};

#[derive(Debug, Clone, PartialEq)]
pub struct PMapFile {
    header: Vec<u8>,
    size: (u32, u32),
    plain: bool,
    pub body: Vec<u8>,
}

pub fn parsepnm(file: Vec<u8>) -> Result<ImgBuffer, &'static str> {
    println!("{}{}", file[0], file[1]);
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
    println!("hasmaxval = {}", hasmaxval);
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
                    println!("comment in state {}", state);
                    backup = marker;
                    marker = state;
                    state = 0;
                }
                (0, 10 | 13) => {
                    state = marker;
                    marker = backup;
                    println!("return to state {} after comment", state);
                }
                (1, 48..=57) => {
                    println!("entering state {}", marker);
                    state = marker;
                    marker = i;
                }
                (2, 9..=13 | 32) => {
                    width = basic::numfromascii(&file[marker..i]);
                    println!("width found: is {}", width);
                    marker = 3;
                    state = 1;
                }
                (3, 9..=13 | 32) => {
                    height = basic::numfromascii(&file[marker..i]);
                    println!("height found: is {}", height);
                    if hasmaxval {
                        marker = 4;
                        state = 1;
                    } else {
                        marker = i + 1;
                        break 'header;
                    }
                }
                (4, 9..=13 | 32) => {
                    cdepth = basic::numfromascii(&file[marker..i]);
                    println!("maxval found: is {}", cdepth);
                    marker = i + 1;
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
    println!("body starts at byte {} which is {}", marker, file[marker]);
    let body = match file[1] {
        49 => readbody::rdbodypbmplain(file, marker),
        50 => readbody::rdbodypgmplain(file, marker, cdepth),
        51 => readbody::rdbodyppmplain(file, marker, cdepth),
        52 => readbody::rdbodypbmraw(file, marker, (width, height)),
        53 => readbody::rdbodypgmraw(file, marker, (width, height), cdepth),
        54 => readbody::rdbodyppmraw(file, marker, (width, height), cdepth),
        _ => return Err("this is not a valid netpbm format"),
    };

    Ok(ImgBuffer {
        size: (width, height),
        ctype,
        body,
    })
}

mod readbody {
    use crate::basic;

    pub fn rdbodypbmraw(file: Vec<u8>, start: usize, size: (u32, u32)) -> Vec<u8> {
        let mut paddedwidth = size.0 as usize / 8;
        let taillen = ((size.0 - 1) % 8) + 1;
        if taillen != 8 {
            paddedwidth += 1;
        }
        let mut body: Vec<u8> = vec![];
        for (i, byte) in file[start..].iter().enumerate() {
            let b = *byte;
            if i % paddedwidth == paddedwidth - 1 {
                for j in 0..taillen {
                    body.push((1 - ((b >> (7 - j)) & 1)) * 255);
                }
            } else {
                for j in 0..8 {
                    body.push((1 - ((b >> (7 - j)) & 1)) * 255);
                }
            }
        }
        body
    }

    pub fn rdbodypgmraw(file: Vec<u8>, start: usize, size: (u32, u32), maxval: u32) -> Vec<u8> {
        let mut body: Vec<u8> = vec![];
        let double = maxval > 255;
        for (i, byte) in file[start..(start + size.0 as usize * size.1 as usize)]
            .iter()
            .enumerate()
        {
            if !double || i % 2 == 0 {
                body.push(*byte);
            }
        }
        body
    }

    pub fn rdbodyppmraw(file: Vec<u8>, start: usize, size: (u32, u32), maxval: u32) -> Vec<u8> {
        let mut body: Vec<u8> = vec![];
        let double = maxval > 255;
        for (i, byte) in file[start..(start + 3 * size.0 as usize * size.1 as usize)]
            .iter()
            .enumerate()
        {
            if !double || i % 2 == 0 {
                body.push(*byte);
            }
        }
        body
    }

    pub fn rdbodypbmplain(file: Vec<u8>, start: usize) -> Vec<u8> {
        let mut body: Vec<u8> = vec![];
        for byte in (file[start..]).iter() {
            if *byte == 48 || *byte == 49 {
                body.push(255 * (49 - *byte));
            }
        }
        body
    }

    pub fn rdbodypgmplain(file: Vec<u8>, start: usize, maxval: u32) -> Vec<u8> {
        let double = maxval > 255;
        let mut body: Vec<u8> = vec![];
        let mut state: char = 'w';
        let mut marker = 0;
        let mut count = 0;
        for (i, byte) in (file[start..]).iter().enumerate() {
            match (state, *byte) {
                ('w', 48..=57) => {
                    marker = i;
                    state = 'n'
                }
                ('n', 9..=13 | 32) => {
                    if count % 2 == 0 || !double {
                        body.push(basic::numfromascii(&file[(marker + start)..(i + start)]) as u8);
                    }
                    state = 'w';
                    count += 1;
                }
                ('w' | 'n', _) => {}
                (_, _) => panic!("you somehow broke this sort-of state machine"),
            }
        }
        body
    }

    pub fn rdbodyppmplain(file: Vec<u8>, start: usize, maxval: u32) -> Vec<u8> {
        let double = maxval > 255;
        let mut body: Vec<u8> = vec![];
        let mut state: char = 'w';
        let mut marker = 0;
        let mut count = 0;
        for (i, byte) in (file[start..]).iter().enumerate() {
            match (state, *byte) {
                ('w', 48..=57) => {
                    marker = i;
                    state = 'n'
                }
                ('n', 9..=13 | 32) => {
                    if count % 2 == 0 || !double {
                        body.push(basic::numfromascii(&file[marker..i]) as u8);
                    }
                    state = 'w';
                    count += 1;
                }
                ('w' | 'n', _) => {}
                (_, _) => panic!("you somehow broke this sort-of state machine"),
            }
        }
        body
    }
}

pub fn assemblepmfile(filestruct: PMapFile) -> Vec<u8> {
    let mut file: Vec<u8> = vec![];
    for byte in filestruct.header {
        file.push(byte);
    }
    file.push(10);
    for digit in basic::makeascii(filestruct.size.0 as usize) {
        file.push(digit);
    }
    file.push(32);
    for digit in basic::makeascii(filestruct.size.1 as usize) {
        file.push(digit);
    }
    file.push(10);
    for byte in filestruct.body {
        file.push(byte);
    }
    file.push(10);
    file
}
