use crate::{basic, pmap};

#[test]
fn testbitpack() {
    let bytes: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
    let byte: u8 = 255;
    assert_eq!(basic::packbits(&bytes[..]), byte);

    let bytes: Vec<u8> = vec![1, 0, 1, 0, 1, 0, 1, 0];
    let byte: u8 = 170;
    assert_eq!(basic::packbits(&bytes[..]), byte);

    let bytes: Vec<u8> = vec![1, 1, 1, 1, 1, 0, 0, 1];
    let byte: u8 = 249;
    assert_eq!(basic::packbits(&bytes[..]), byte);

    let bytes: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 1];
    let byte: u8 = 129;
    assert_eq!(basic::packbits(&bytes[..]), byte);

    let bytes: Vec<u8> = vec![1, 1, 1, 1, 1];
    let byte: u8 = 248;
    assert_eq!(basic::packbits(&bytes[..]), byte);
}

#[test]
fn testrowpack() {
    let row: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];
    let bytes: Vec<u8> = vec![255, 0];
    assert_eq!(bytes, basic::packrow(&row[..]));

    let row: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1];
    let bytes: Vec<u8> = vec![1, 199];
    assert_eq!(bytes, basic::packrow(&row[..]));

    let row: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1];
    let bytes: Vec<u8> = vec![0, 25];
    assert_eq!(bytes, basic::packrow(&row[..]));

    let row: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0];
    let bytes: Vec<u8> = vec![1, 32];
    assert_eq!(bytes, basic::packrow(&row[..]));

    let row: Vec<u8> = vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0];
    let bytes: Vec<u8> = vec![255, 0];
    assert_eq!(bytes, basic::packrow(&row[..]));
}

#[test]
fn pnmgrayfileparsing() {
    use std::fs;
    // checks the same file as different grayscale pnm files returns the same ImgBuffer
    let plainfilepgm = pmap::parsepnm(fs::read("Pplain.pgm").unwrap()).unwrap();
    let plainfilepbm = pmap::parsepnm(fs::read("Pplain.pbm").unwrap()).unwrap();
    let rawfilepgm = pmap::parsepnm(fs::read("Praw.pgm").unwrap()).unwrap();
    let rawfilepbm = pmap::parsepnm(fs::read("Praw.pbm").unwrap()).unwrap();
    assert_eq!(plainfilepbm, rawfilepbm);
    assert_eq!(rawfilepbm, rawfilepgm);
    assert_eq!(plainfilepgm, rawfilepgm);
}

#[test]
fn colourandback() {}
