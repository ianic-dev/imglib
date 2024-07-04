use imglib::*;
use pmap::writepnm;
use std::fs;

fn main() {
    let img = pmap::makepbm(
        pmap::parsepnm(fs::read("Pplain.pbm").unwrap()).unwrap(),
        Some(128),
    );
    let file = writepnm(img, true);
    let _ = fs::write("out.pbm", file);
}
