use std::fs::*;
use std::io::{self, Write};
use std::path::Path;

pub fn write<P: AsRef<Path>>(filename: P, s: &str) -> io::Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(s.as_bytes())?;
    Ok(())
}

pub fn mkdir<P: AsRef<Path>>(filename: P) -> Result<(), io::Error> {
    match create_dir(filename) {
        Err(e) => Err(e),
        Ok(_) => Ok(()),
    }
}