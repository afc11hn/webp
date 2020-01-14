use std::env;

use std::fs::File;
use std::io::{Read, Write};
use std::ops::Deref;
use webp::Encoder;
use std::path::PathBuf;

fn main() {
    for filename in env::args().skip(1) {
        let mut data = vec![];
        File::open(&filename).unwrap().read_to_end(&mut data).unwrap();

        let image = image::load_from_memory(&data).unwrap();
        let webp_decoder = Encoder::from_image(&image);
        let webp_image = webp_decoder.encode(100.0);

        File::create(new_filename(&filename)).unwrap()
            .write(webp_image.deref()).unwrap();
    }
}

fn new_filename(path: &str) -> PathBuf {
    let path = PathBuf::from(path);
    let filename = path.file_stem().unwrap().to_str().unwrap();
    path.with_file_name(format!("{}.webp", filename))
}