use std::io::{self, Write};

extern crate glob;
use glob::glob;

mod game;
use game::Game;
use game::GameTree;

fn main() {
    let mut moves = Vec::new();
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
    
    loop {
        print!("> "); flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().as_ref() {
            "?" => help(),
            "m" => println!("Moves: {:?}", moves),
            "r" => println!("{:?}", gt.traverse(&moves).results),
            "t" => for (n, m) in gt.traverse(&moves).top_ten() {
                println!("{}\t\t{}", m, n);
            },
            "x" => {
                print!("x> "); flush();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                gt.traverse_mut(&moves).expand(input.trim());
                moves.push(String::from(input.trim()));
            },
            "p" => {
                moves.pop();
            },
            "q" => break,
            _ => error(),
        }
    }
}

fn help() {
    println!("? -> print this message");
    println!("m -> print moves");
    println!("r -> print results");
    println!("t -> print top ten moves");
    println!("x -> expand one move");
    println!("p -> pop last move");
    println!("q -> quit");
}

fn error() {
    println!("Unrecognized command, for help: ?");
}

fn flush() {
    io::stdout().flush().unwrap();
}
