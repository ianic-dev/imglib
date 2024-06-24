use imglib::*;
use std::fs;

fn main() {
    let file = fs::read("test.pbm").unwrap();
    /* if file[0..2] == vec![80, 49] {
        let (size, mut data) = pbm_plain(file);
        println!("{}", data.len());
    } */
    let file = separate_header(file);
    let packfile = packpbm(file);
    println!("{packfile:?}");
    let mut outfile: Vec<u8> = vec![80, 52, 10, 49, 54, 32, 49, 54];
    let mut count = 0;
    for byte in packfile.body {
        if count % 16 == 0 {
            outfile.push(10)
        }
        outfile.push(byte);
        count = (count + 1) % 16
    }
    println!("{outfile:?}");
    let _ = fs::write("out.pbm", outfile);
}
