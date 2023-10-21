use std::fs;
use std::io::prelude::*;

pub fn read(filename: String) -> String {
    let mut f = fs::File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

pub fn mkdir(filename: String) -> u8 {
    match fs::create_dir(filename.clone()) {
        Err(e) => panic!("{}: {}", filename, e),
        Ok(_) => 0,
    }
}