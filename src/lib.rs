mod tests;

pub struct RGB(u8, u8, u8);

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

#[derive(Debug)]
pub struct PMapFile {
    header: [u8; 2],
    size: (usize, usize),
    plain: bool,
    pub body: Vec<u8>,
}

pub fn separate_header(rawfile: Vec<u8>) -> PMapFile {
    let header: [u8; 2] = (&rawfile[0..2]).try_into().expect("header failed to read");
    let filestr = String::from_utf8_lossy(&rawfile).to_string();
    let mut fileiter = filestr.split(|c| char::is_whitespace(c));
    fileiter.next();
    let size: (usize, usize) = (
        fileiter.next().unwrap().parse().expect("invalid width"),
        fileiter.next().unwrap().parse().expect("invalid height"),
    );
    let mut body: Vec<u8> = vec![];
    for n in fileiter {
        if n != "" {
            body.push(n.parse().expect("haha L"));
        }
    }
    PMapFile {
        header,
        size,
        plain: false,
        body,
    }
}

pub fn packbits(bits: &[u8]) -> u8 {
    let mut byte = 0;
    for exp in 0..8 {
        let cbit = if exp < bits.len() { bits[exp] } else { 0 };
        byte += cbit * (2_u8.pow((7 - exp) as u32) as u8);
        //println!("bit {} exp {} byte {}", cbit, 7 - exp, byte);
    }
    byte
}

pub fn packrow(row: &[u8]) -> Vec<u8> {
    let mut length = row.len() / 8;
    if row.len() % 8 == 0 {
        length += 1;
    }
    let mut packedrow: Vec<u8> = vec![];
    for i in 0..length {
        packedrow.push(packbits(&row[i..(i + 8)]));
    }
    packedrow
}

pub fn packpbm(mut file: PMapFile) -> PMapFile {
    let mut packedbody: Vec<u8> = vec![];
    for y in 0..file.size.1 {
        let row = packrow(&file.body[file.size.0 * y..file.size.0 * (y + 1)]);
        for elem in row {
            packedbody.push(elem);
        }
    }
    file.body = packedbody;
    file.plain = false;
    file
}

pub fn pack(plainfile: PMapFile) -> PMapFile {
    if plainfile.header == [80, 49] {
        packpbm(plainfile)
    } else {
        plainfile
    }
}
