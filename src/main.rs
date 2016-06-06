
extern crate glob;
use glob::glob;

use std::fs::File;
use std::path::Path;
//use std::io::prelude::*;
//use std::error::Error;
use std::io::{BufRead, BufReader};

fn main() {
    for entry in glob("data/*.pgn").unwrap() {
        match entry {
            Ok(path) => load_pgn(path.as_path()),
            Err(e)   => println!("{:?}", e),
        }
    }

}

fn load_pgn(path: &Path) {
    let display = path.display();
    let file = match File::open(path) {
        Ok(f) => f,
        _     => panic!("opening {}", display),
    };
    let reader = BufReader::new(file);
    let mut header = String::new();
    let mut moves = String::new();
    let mut reading_header = true;
    for l in reader.lines() {
        let line = match l {
            Ok(ln) => ln,
            _      => panic!("reading {}", display),
        };
        if line == "" {
            reading_header = !reading_header;
            if reading_header {
                parse_game(&header, &moves);
                header = String::new();
                moves = String::new();
            }
            continue;
        }
        if reading_header {
            header.push_str(&line);
        } else {
            moves.push_str(&line);
        }
    }
    println!("{:?}", display); // remove in future
}

fn parse_game(header: &String, moves: &String) {
    // TODO
}
