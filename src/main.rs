use imglib::*;
use pmap::writepnm;
use std::fs;

fn main() {
    let img = pmap::makepgm(pmap::parsepnm(fs::read("Pplain.pgm").unwrap()).unwrap());
    let file = writepnm(img, true);
    let _ = fs::write("out.pgm", file);
}
