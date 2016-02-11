extern crate glob;
use glob::glob;

//use std::error::Error;
//use std::fs::File;
//use std::io::prelude::*;
use std::path::Path;

fn main() {
    for entry in glob("data/*.pgn").unwrap() {
        match entry {
            Ok(path) => load_pgn(path.as_path()),
            Err(e)   => println!("{:?}", e),
        }
    }
}

fn load_pgn(path: &Path) {
    println!("{:?}", path.display());
}
