use std::fs;

use crate::{
    basic,
    pmap::{self, makepbm},
};

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
    let plainfilepgm = pmap::parsepnm(fs::read("testimg/Pplain.pgm").unwrap()).unwrap();
    let plainfilepbm = pmap::parsepnm(fs::read("testimg/Pplain.pbm").unwrap()).unwrap();
    let rawfilepgm = pmap::parsepnm(fs::read("testimg/Praw.pgm").unwrap()).unwrap();
    let rawfilepbm = pmap::parsepnm(fs::read("testimg/Praw.pbm").unwrap()).unwrap();
    assert_eq!(plainfilepbm, rawfilepbm);
    assert_eq!(rawfilepbm, rawfilepgm);
    assert_eq!(plainfilepgm, rawfilepgm);
}

#[test]
fn readwrite() {
    let plainfilepgm = fs::read("testimg/Pplain.pgm").unwrap();
    let plainfilepbm = fs::read("testimg/Pplain.pbm").unwrap();
    let rawfilepgm = fs::read("testimg/Praw.pgm").unwrap();
    let rawfilepbm = fs::read("testimg/Praw.pbm").unwrap();
    assert_eq!(
        plainfilepgm,
        pmap::writepnm(
            pmap::makepgm(pmap::parsepnm(plainfilepgm.clone()).unwrap()),
            true
        )
    );
    assert_eq!(
        plainfilepbm,
        pmap::writepnm(
            pmap::makepbm(pmap::parsepnm(plainfilepbm.clone()).unwrap(), Some(128)),
            true
        )
    );
    assert_eq!(
        rawfilepgm,
        pmap::writepnm(
            pmap::makepgm(pmap::parsepnm(plainfilepgm.clone()).unwrap()),
            false
        )
    );
    assert_eq!(
        rawfilepbm,
        pmap::writepnm(
            pmap::makepbm(pmap::parsepnm(plainfilepbm.clone()).unwrap(), Some(128)),
            false
        )
    );
}

#[test]
fn imgbufferlen() {
    let testimg = pmap::parsepnm(fs::read("testimg/colourtest.ppm").unwrap()).unwrap();
    let testlen = testimg.body.len();
    let pimg = testimg.tograyscale();
    let len2 = pimg.body.len();
    assert_eq!(testlen, len2 * 3);
    let pbm = makepbm(pimg, Some(128));
    assert_eq!(len2, pbm.body.len())
}

#[test]
fn colourtest() {
    let testraw = pmap::parsepnm(fs::read("testimg/Praw.ppm").unwrap()).unwrap();
    let testplain = pmap::parsepnm(fs::read("testimg/Pplain.ppm").unwrap()).unwrap();
    assert_eq!(testraw, testplain);
}
