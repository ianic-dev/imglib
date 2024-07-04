use std::usize;

use crate::basic;

use super::PNMap;

pub fn mkbodypbmplain(image: PNMap) -> Vec<u8> {
    let mut body: Vec<u8> = vec![];
    let mut c = 1;
    for b in image.body {
        body.push(b + 48);
        if c % 70 == 0 {
            body.push(10);
        }
        c += 1;
    }
    body.push(10);
    body
}

pub fn mkbodypgmplain(image: PNMap) -> Vec<u8> {
    let mut body: Vec<u8> = vec![];
    for n in image.body {
        let number = basic::makeascii(n as u32);
        for b in number {
            body.push(b);
        }
        body.push(10);
    }
    body
}

pub fn mkbodyppmplain(image: PNMap) -> Vec<u8> {
    let mut body: Vec<u8> = vec![];
    for n in image.body {
        let number = basic::makeascii(n as u32);
        for b in number {
            body.push(b);
        }
        body.push(10);
    }
    body
}

pub fn mkbodypbmraw(image: PNMap) -> Vec<u8> {
    println!("{:?}", image.body.len());
    let origbody = &image.body[..];
    let mut body: Vec<u8> = vec![];
    for i in 0..(image.size.1 as usize) {
        let row = &origbody[(i * (image.size.0 as usize))..((i + 1) * (image.size.0 as usize))];
        let row = basic::packrow(&row);
        println!("{}", row.len());
        for b in row {
            body.push(b);
        }
    }
    body
}

pub fn mkbodypgmraw(image: PNMap) -> Vec<u8> {
    let mut body = vec![];
    for b in image.body {
        body.push(b);
    }
    body
}

pub fn mkbodyppmraw(image: PNMap) -> Vec<u8> {
    let mut body = vec![];
    for b in image.body {
        body.push(b);
    }
    body
}
