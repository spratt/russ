extern crate glob;
use glob::glob;

mod game;
use game::Game;

fn main() {
    let mut v = Vec::new();
    for entry in glob("data/*.pgn").unwrap() {
        match entry {
            Ok(path) => v.extend(Game::from_pgn(path.as_path()).iter().cloned()),
            Err(e)   => println!("{:?}", e),
        }
    }
}
