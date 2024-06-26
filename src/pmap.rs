use crate::basic;
#[derive(Debug, Clone, PartialEq)]
pub struct PMapFile {
    header: [u8; 2],
    size: (usize, usize),
    plain: bool,
    pub body: Vec<u8>,
}

pub fn pbm_plain(mut file: Vec<u8>) -> ((usize, usize), Vec<u8>) {
    let filestr = String::from_utf8(file.clone()).unwrap();
    // println!("{filestr}");
    let filesplt = filestr.split(&[' ', '\n', '\t']);
    let mut filevec = vec![];
    for elem in filesplt {
        filevec.push(elem);
    }
    let size: (usize, usize) = (
        filevec[1].parse().expect("no valid width found"),
        filevec[2].parse().expect("no valid height found"),
    );
    file.retain(|x| (*x == 48 || *x == 49));
    let mut outfile: Vec<u8> = vec![];
    for i in (file.len() - (size.0 * size.1))..file.len() {
        // println!("{i}");
        outfile.push(file[i] - 48);
    }
    (size, outfile)
}

pub fn parse_pmfile(rawfile: Vec<u8>) -> PMapFile {
    let header: [u8; 2] = (&rawfile[0..2]).try_into().expect("header failed to read");
    let filestr = String::from_utf8_lossy(&rawfile).to_string();
    let mut fileiter = filestr.split(|c| char::is_whitespace(c)).enumerate();
    fileiter.next();
    let size: (usize, usize) = (
        (fileiter.next().unwrap())
            .1
            .parse()
            .expect("invalid width "),
        (fileiter.next().unwrap())
            .1
            .parse()
            .expect("invalid height"),
    );
    let (bodyindex, _) = fileiter
        .next()
        .expect("file has no detected body, is not long enough");
    println!("bodyindex {}", bodyindex);
    let plain = header[1] & 1 != 0;
    let body: Vec<u8> = if !plain {
        parseraw(size.0 * size.1, rawfile)
    } else {
        parseplain(bodyindex, rawfile)
    };
    PMapFile {
        header,
        size,
        plain,
        body,
    }
}
fn parseplain(bodyindex: usize, rawfile: Vec<u8>) -> Vec<u8> {
    let mut body: Vec<u8> = vec![];
    let filestr = String::from_utf8_lossy(&rawfile).to_string();
    let fileiter = filestr.split(|c| char::is_whitespace(c));
    for (ind, byte) in fileiter.enumerate() {
        let mut valid = true;
        for c in byte.as_bytes() {
            if (*c as char).is_whitespace() {
                valid = false;
            }
        }
        println!("char: {}, valid: {}", byte, valid);
        if ind >= bodyindex && valid && (byte != "") {
            let n: u8 = byte.parse().expect("Invalid ASCII number in body");
            body.push(n)
        }
    }
    body
}
fn parseraw(length: usize, rawfile: Vec<u8>) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    let mut state: u8 = 0;
    let mut start: usize = 0;
    for (i, byte) in rawfile.iter().enumerate() {
        if i >= start + length {
            break;
        }
        match (state, *byte) {
            (8.., b) => out.push(b),
            (7.., c) => {
                out.push(c);
                start = i;
                state += 1
            }
            (0, 0x50)
            | (1, 48..=57)
            | (2, 9..=13 | 32)
            | (3, 48..=57)
            | (4, 9..=13 | 32)
            | (5, 48..=57)
            | (6, 9..=13 | 32) => state += 1,
            (_, _) => {}
        }
    }
    out
}

pub fn assemblepmfile(filestruct: PMapFile) -> Vec<u8> {
    let mut file: Vec<u8> = vec![];
    for byte in filestruct.header {
        file.push(byte);
    }
    file.push(10);
    for digit in basic::makeascii(filestruct.size.0) {
        file.push(digit);
    }
    file.push(32);
    for digit in basic::makeascii(filestruct.size.1) {
        file.push(digit);
    }
    file.push(10);
    for byte in filestruct.body {
        file.push(byte);
    }
    file.push(10);
    file
}

pub fn packpmap(plainfile: PMapFile) -> PMapFile {
    if plainfile.header == [80, 49] {
        packpbm(plainfile)
    } else {
        plainfile
    }
}

pub fn packpbm(mut file: PMapFile) -> PMapFile {
    let mut packedbody: Vec<u8> = vec![];
    for y in 0..file.size.1 {
        let row = basic::packrow(&file.body[file.size.0 * y..file.size.0 * (y + 1)]);
        for elem in row {
            packedbody.push(elem);
        }
    }
    file.body = packedbody;
    file.plain = false;
    file
}
