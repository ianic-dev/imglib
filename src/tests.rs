#[test]
fn testbitpack() {
    use crate::basic;
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
    use crate::basic;
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
fn rawpbmfileparsing() {
    use crate::pmap;
    use std::fs;
    // packing a packed pbm does not alter it
    let file = pmap::parse_pmfile(fs::read("testraw.pbm").unwrap());
    let file2 = pmap::packpmap(file.clone());
    assert_eq!(file, file2);
}
#[test]
fn placeholdername() {
    use crate::pmap;
    let v = pmap::parsepmap(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0]);
    panic!();
}
