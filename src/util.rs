use std::fs;
use std::io::prelude::*;

pub fn read(filename: String) -> String {
    let mut f = fs::File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

pub fn write(filename: String, s: &String) {
    let mut f = fs::File::create(filename).unwrap();

    _ = f.write_all(s.as_bytes());
}

pub fn mkdir(filename: String) -> u8 {
    match fs::create_dir(filename.clone()) {
//        Err(e) => panic!("{}: {}", filename, e),
        Err(_) => 1,
        Ok(__) => 0,
    }
}