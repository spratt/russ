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
Parsing data files..done.
? -> print this message
m -> print moves
r -> print results
t -> print top ten moves
x -> expand
q -> quit
> ?
? -> print this message
m -> print moves
r -> print results
t -> print top ten moves
x -> expand
q -> quit
> m
Moves: []
> r
Results { white: 5963, black: 4965, draw: 4433 }
> t
e4		9290
d4		4832
Nf3		485
c4		448
f4		184
e3		61
a3		13
g3		12
b3		9
> x
Not yet implemented.
> q
```

Warning
=======

This utility does not correctly parse PGN files, only a simple
approximation.  For example, {comments} are assumed to begin a line
and last until the end of the line, as in:

```
{This is a successfully parsed comment.}
```

But the following will not parse correctly:

```
 {This line begins with a space and continues into the next line,
which breaks my assumptions.}
```
