Russ
----

A chess utility written in Rust.

Chess data courtesy of http://www.hoflink.com/~npollock/chess.html
distributed in accordance with the readme.

Usage
=====

Call `cargo run` from the command line in the project's root
directory.  Russ will read all the `*.pgn` files in `data/`.  By
default, only one file is kept in `data/` and the rest are in
`more_data/`.  This is for debug purposes, since it takes forever to
parse everything.  For production use, move all `*.pgn` files into
`data/`.

```bash
$ cargo run
     Running `target/debug/russ`
Parsing data files.......done.
Results { white: 114649, black: 73136, draw: 145324 }
e4		142263
d4		131608
Nf3		29319
c4		25478
g3		2046
b3		853
f4		773
Nc3		283
b4		149
```

Warning
=======

This utility does not correctly parse PGN files, only a simple
approximation.  For example, {comments} are assumed to begin a line
and last until the end of the line, as in:

{This is a successfully parsed comment.}

But the following will not parse correctly:

 {This line begins with a space and continues into the next line,
which breaks my assumptions.}
