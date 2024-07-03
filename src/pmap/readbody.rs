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
