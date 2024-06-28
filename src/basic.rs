use std::usize;

pub fn makeascii(mut num: usize) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    while num > 0 {
        out.push((num % 10 + 48) as u8);
        num /= 10;
    }
    out.reverse();
    out
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
    if row.len() % 8 != 0 {
        length += 1;
    }
    //println!("rowlen {} packedlen {}", row.len(), length);
    let mut packedrow: Vec<u8> = vec![];
    for i in 0..length {
        if 8 * (1 + i) <= row.len() {
            packedrow.push(packbits(&row[(8 * i)..(8 * (i + 1))]));
        } else {
            packedrow.push(packbits(&row[8 * i..row.len()]));
        }
    }
    packedrow
}

pub fn numfromascii(bytes: &[u8]) -> u32 {
    let mut n = 0;
    for b in bytes {
        n += (*b as u32) - 48;
        n *= 10;
    }
    n / 10
}
