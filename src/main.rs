use std::io::{self, Write};

extern crate glob;
use glob::glob;

mod game;
use game::Game;
use game::GameTree;

fn main() {
    let mut gt = GameTree::new();
    print!("Parsing data files.");
    io::stdout().flush().unwrap();
    for entry in glob("data/*.pgn").unwrap() {
        match entry {
            Ok(path) => gt.add(Game::from_pgn(path.as_path())),
            Err(e)   => println!("{:?}", e),
        }
        print!(".");
        io::stdout().flush().unwrap();
    }
    println!("done.");
    println!("{:?}", gt.results);
    for (n, m) in gt.top_ten() {
        println!("{}\t\t{}", m, n);
    }
}
