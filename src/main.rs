use std::io::{self, Write};

extern crate glob;
use glob::glob;

mod game;
use game::Game;
use game::GameTree;

fn main() {
    let mut gt = GameTree::new();
    print!("Parsing data files."); flush();
    for entry in glob("data/*.pgn").unwrap() {
        match entry {
            Ok(path) => gt.add(Game::from_pgn(path.as_path())),
            Err(e)   => println!("{:?}", e),
        }
        print!("."); flush();
    }
    println!("done.");
    // begin CLI
    help();
    while true {
        print!("> "); flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => {
                println!("error: {}", error);
                break;
            }
        }
        match input.as_ref() {
            "?\n" => help(),
            "m\n" => println!("Moves: {:?}", gt.moves),
            "r\n" => println!("{:?}", gt.results),
            "t\n" => for (n, m) in gt.top_ten() {
                println!("{}\t\t{}", m, n);
            },
            "x\n" => println!("Not yet implemented."),
            "q\n" => break,
            _ => error(),
        }
    }
}

fn help() {
    println!("? -> print this message");
    println!("m -> print moves");
    println!("r -> print results");
    println!("t -> print top ten moves");
    println!("x -> expand");
    println!("q -> quit");
}

fn error() {
    println!("Unrecognized command, for help: ?");
}

fn flush() {
    io::stdout().flush().unwrap();
}
