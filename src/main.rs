extern crate glob;
use glob::glob;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

mod game;
use game::Game;

fn main() {
    let mut v = Vec::new();
    for entry in glob("data/*.pgn").unwrap() {
        match entry {
            Ok(path) => v.extend(load_pgn(path.as_path()).iter().cloned()),
            Err(e)   => println!("{:?}", e),
        }
    }
}

fn load_pgn(path: &Path) -> Vec<Game> {
    let mut v = Vec::new();
    let display = path.display();
    let file = match File::open(path) {
        Ok(f) => f,
        _     => panic!("opening {}", display),
    };
    let reader = BufReader::new(file);
    let mut header = Vec::new();
    let mut moves = Vec::new();
    let mut reading_header = true;
    for l in reader.lines() {
        let line = match l {
            Ok(ln) => ln,
            _      => panic!("reading {}", display),
        };
        if line == "" {
            reading_header = !reading_header;
            if reading_header {
                v.push(Game::new(&header, &moves));
                header = Vec::new();
                moves = Vec::new();
            }
            continue;
        }
        if reading_header {
            header.push(line);
        } else {
            moves.push(line);
        }
    }
    println!("{:?}", display); // remove in future
    v
}
