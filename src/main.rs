// Attempt #4
// #![feature(io)]
// #![feature(path)]
// use std::old_io::File;

// Attempt #1
//extern crate glob;
//use glob::glob;

// Attempt #3
// use std::str::from_utf8_owned;
// use std::fs::File;

// Attempts #1 and #2
// use std::error::Error;
// use std::path::Path;
// use std::io;
// use std::io::prelude::*;
// use std::io::BufReader;

fn main() {
    let mut file = std::fs::File::open("data/gm1931.pgn").unwrap();
    let mut stdout = std::io::stdout();
    std::io::copy(&mut file, &mut stdout).unwrap();

    // Attempt #4
    // // Create a path to the desired file
    // let path = Path::new("data/gm1931.pgn");
    // let display = path.display();
    // // Open the path in read-only mode, returns `IoResult<File>`
    // let mut file = match File::open(&path) {
    //     // The `desc` field of `IoError` is a string that describes the error
    //     Err(why) => panic!("couldn't open {}: {}", display, why.desc),
    //     Ok(file) => file,
    // };
    // // Read the file contents into a string, returns `IoResult<String>`
    // match file.read_to_string() {
    //     Err(why) => panic!("couldn't read {}: {}", display, why.desc),
    //     Ok(string) => print!("{} contains:\n{}", display, string),
    // }

    // Attempt #1
    // for entry in glob("data/*.pgn").unwrap() {
    //     match entry {
    //         Ok(path) => load_pgn(path.as_path()),
    //         Err(e)   => println!("{:?}", e),
    //     }
    // }

    // Attempt #2
    // fn foo() -> io::Result<()> {
    //     let f = try!(File::open("data/gm1931.pgn"));
    //     let mut reader = BufReader::new(f);
    //     let mut buffer = String::new();
        
    //     // read a line into buffer
    //     try!(reader.read_line(&mut buffer));
        
    //     println!("{}", buffer);
    //     Ok(())
    // }
    // Ok(foo());

    // Attempt #3
    // let path = Path::new("data/gm1931.pgn");
    // let mut hw_file = File::open(&path);
    // let data = from_utf8_owned(hw_file.read_to_end());
    // println!("{}", data);
}

// Attempt #1
// fn load_pgn(path: &Path) -> Result<(), Error> {
//     // println!("{:?}", path.display());
// }
