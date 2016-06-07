Russ
----

A chess utility written in Rust.

Chess data courtesy of http://www.hoflink.com/~npollock/chess.html
distributed in accordance with the readme.

Usage
=====

Call `cargo run` from the command line in the project's root
directory.  Russ will read all the `*.pgn` files in `data/`.  For
debugging, I recommend moving all but `gm1830.pgn` out of data, to
reduce loading time.

```bash
$ cargo run
     Running `target/debug/russ`
Parsing data files.......done.
? -> print this message
m -> print moves
r -> print results
t -> print top ten moves
x -> expand one move
p -> pop last move
q -> quit
> t
e4         142263	white: 34.16, black: 23.10, draw: 42.74
d4         131608	white: 34.70, black: 20.76, draw: 44.54
Nf3         29319	white: 33.57, black: 21.69, draw: 44.74
c4          25478	white: 35.92, black: 20.99, draw: 43.09
g3           2046	white: 33.77, black: 24.88, draw: 41.35
b3            853	white: 33.06, black: 27.32, draw: 39.62
f4            773	white: 30.53, black: 34.15, draw: 35.32
Nc3           283	white: 27.21, black: 21.20, draw: 51.59
b4            149	white: 23.49, black: 28.19, draw: 48.32
> r
Results { white: 114649, black: 73136, draw: 145324 }
> x
x> e4
> r
Results { white: 48593, black: 32869, draw: 60801 }
> t
c5          60488	white: 33.93, black: 24.44, draw: 41.63
e5          42771	white: 32.91, black: 22.00, draw: 45.09
e6          16584	white: 35.72, black: 22.67, draw: 41.61
c6          11235	white: 33.40, black: 20.96, draw: 45.64
d6           4051	white: 39.15, black: 22.59, draw: 38.26
g6           2699	white: 38.16, black: 23.68, draw: 38.16
d5           1986	white: 38.52, black: 21.00, draw: 40.48
Nf6          1986	white: 37.26, black: 23.01, draw: 39.73
Nc6           319	white: 37.93, black: 31.35, draw: 30.72
> m
Moves: ["e4"]
> p
> m
Moves: []
> q
```

Note that the above output shows that of the ~200,000 games for which
there is included data, 142,263 of them start with white moving to `e4`.

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
