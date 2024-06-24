use imglib::*;
use std::fs;

fn main() {
    let file = fs::read("testraw.pbm").unwrap();
    /* if file[0..2] == vec![80, 49] {
        let (size, mut data) = pbm_plain(file);
        println!("{}", data.len());
    } */
    let file = imglib::pmap::parse_pmfile(file);
    dbg!(file);
}
