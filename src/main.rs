use imglib::*;
use pmap::writepnm;
use std::fs;

fn main() {
    let img = pmap::makepbm(
        pmap::parsepnm(fs::read("testimg/Pplain.pbm").unwrap()).unwrap(),
        Some(128),
    );
    let file = writepnm(img, true);
    let _ = fs::write("testimg/out.pbm", file);
}
