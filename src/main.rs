extern crate glob;
use glob::glob;

mod game;
use game::Game;
use game::GameTree;

fn main() {
    let mut gt = GameTree::new();
    for entry in glob("data/*.pgn").unwrap() {
        match entry {
            Ok(path) => gt.add(Game::from_pgn(path.as_path())),
            Err(e)   => println!("{:?}", e),
        }
    }
    println!("{:?}", gt.results);
}
