extern crate unicode_segmentation;
use game::unicode_segmentation::UnicodeSegmentation;

use std::path::Path;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub struct Results {
    white: u64,
    black: u64,
    draw: u64,
}

impl Clone for Results {
    fn clone(&self) -> Results {
        Results {
            white: self.white.clone(),
            black: self.black.clone(),
            draw: self.draw.clone(),
        }
    }
}

pub struct Game {
    moves: Vec<String>,
    tagpairs: HashMap<String, String>,
    results: Results,
}

impl Clone for Game {
    fn clone(&self) -> Game {
        Game {
            moves: self.moves.clone(),
            tagpairs: self.tagpairs.clone(),
            results: self.results.clone(),
        }
    }
}

fn remove_first_last(s: &String) -> String {
    s.graphemes(true).skip(1).take(s.len() - 1).collect::<String>()
}

impl Game {
    fn parse_moves(moves: &Vec<String>) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        for line in moves {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for part in parts {
                if !(String::from(part).ends_with(".")) {
                    v.push(String::from(part));
                }
            }
        }
        v.clone()
    }

    fn parse_tagpairs(tagpairs: &Vec<String>) -> HashMap<String, String> {
        let mut tp: HashMap<String, String> = HashMap::new();
        for s in tagpairs {
            let t = remove_first_last(s);
            let v: Vec<&str> = t.split_whitespace().collect();
            tp.insert(String::from(v[0]), remove_first_last(&String::from(v[1])));
        }
        tp.clone()
    }

    fn parse_results(tagpairs: &HashMap<String, String>) -> Results {
        match tagpairs.get("Result").map(String::as_ref) {
            Some("1-0") => Results { white: 1, black: 0, draw: 0 },
            Some("0-1") => Results { white: 0, black: 1, draw: 0 },
            Some("1/2-1/2") => Results { white: 0, black: 0, draw: 1 },
            _ => Results { white: 0, black: 0, draw: 0 },
        }
    }

    pub fn new(tagpairs: &Vec<String>, moves: &Vec<String>) -> Game {
        let tp = Game::parse_tagpairs(tagpairs);
        Game {
            moves: Game::parse_moves(moves),
            tagpairs: tp.clone(),
            results: Game::parse_results(&tp),
        }
    }

    pub fn from_pgn(path: &Path) -> Vec<Game> {
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
}

pub struct GameTree {
    moves: Vec<String>,
    game_vecs: HashMap<String, Vec<Game>>,
}

impl GameTree {
    pub fn new() -> GameTree {
        GameTree {
            moves: Vec::new(),
            game_vecs: HashMap::new(),
        }
    }
    pub fn from_moves(moves: &Vec<String>) -> GameTree {
        GameTree {
            moves: moves.clone(),
            game_vecs: HashMap::new(),
        }
    }

    pub fn add(&mut self, v: Vec<Game>) {
        for game in v {
            let n = self.moves.len();
            let next_move: &str = game.moves[n].as_ref();
            if !self.game_vecs.contains_key(next_move) {
                self.game_vecs.insert(String::from(next_move), Vec::new());
            }
            self.game_vecs.get_mut(next_move).unwrap().push(game.clone());
        }
    }
}
