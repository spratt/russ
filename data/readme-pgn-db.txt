readme.txt for "40H-PGN" databases.
copyright (c) 2014-15 by Norman Pollock. All rights reserved.

These PGN database files are freeware. They consist of games 
available in the Public Domain on the Internet that were further
edited by Norman Pollock (rc1242(at)yahoo(dot)com). 

No warranty, expressed or implied, is made that the information 
provided in readme-pgn-db.txt is accurate. 

No warranty, expressed or implied, is made that the information in
each game description is accurate. This includes names of players,
game moves and game results.

No warranty, expressed or implied, is made as to the worthiness of 
these files for computer or chess usage. 

The user bears full responsibility for any consequences from the use 
or misuse of these files. The editor is not responsible for any
damages or losses of any kind.

These database files can be freely distributed provided they are 
distributed "as is". The editor is not responsible for any changes 
made by others.

-------------

"40H-PGN-databases" is divided into two sections: "gm" databases 
containing human games, and "cc" databases containing computer
chess (engine-engine) games.

-------------

=================
"gm" databases
=================

High quality databases of long time-control games between top 
Human players. There are 4 files: gm1830.pgn, gm1931.pgn, 
gm1981.pgn and gm2006.pgn.

Double games (same moves as an existing game) and many near-double 
games were excluded.

Certain types of games have been excluded, for example, Rapid and 
Blitz games. Tag data was used to determine such games.

Games were checked for technical errors using "joined" by Andreas 
Stable and "pgn-extract" by David Barnes. Games with errors were 
excluded.

Games were checked for major blunders in the first 25 moves using 
"Game Analyser" by Thomas McBurney. Games with blunders were 
excluded.

"ECO" codes were recalculated using extended "SCID" ECO codes. 

Games have full-move formatting on each line.

Games were sorted in the following order: ECO, WhitePlayer, 
BlackPlayer, Result, Date.

-------------

(1) gm1830.pgn

Games were played from 1834 to 1930. 

No blindfold, simultaneous, exhibition, or cable games. 
Exclusions based on Tag data and might not exclude all such 
games.

Number of games = 15,361
Number of players = 414
Number of clusters = 2 (see note below)
Minimum player score = 20.0%
Minimum Elo = none
Minimum occurrences each player = 8
Minimum plies each game = 51
Beginning date of games = 1834.01.01
Ending date of games = 1930.08.01
Version = 2015.07.15

Note: "De la Bourdonnais, Louis C." and "MacDonnell, Alexander" 
only played each other.

-------------

(2) gm1931.pgn

Games were played from 1931 to 1980. 

No blitz, blindfold, simultaneous, exhibition, radio or telex 
games. Exclusions based on Tag data and might not exclude all 
such games.

Each player has at least 8 games in the database, and a result 
score of at least 20.00%.

Number of games = 47,054
Number of players = 1,073
Number of clusters = 1
Minimum player score = 20.0%
Minimum Elo = none
Minimum occurrences each player = 8
Minimum plies each game = 51
Beginning date of games = 1931.01.01
Ending date of games = 1980.10.01
Version = 2015.07.15

-------------

(3) gm1981.pgn

Games were played from 1981 to 2005. 

No blitz, rapid, blindfold, simultaneous, exhibition, playoff,
KO, Internet or correspondence games. Exclusions based on Tag 
data and might not exclude all such games.

Number of games = 99,980
Number of players = 1,232
Number of clusters = 1
Minimum player score = 20.0%
Minimum Elo = 2450
Minimum occurrences each player = 12
Minimum plies each game = 51
Beginning date of games = 1981.01.01
Ending date of games = 2005.12.31
Version = 2016.02.06

-------------

(4) gm2006.pgn

Games were played from 2006 to 2015. 

No blitz, rapid, tiebreak, blindfold, simultaneous, exhibition, 
playoff, KO, Internet or correspondence games. Exclusions based 
on Tag data and might not exclude all such games.

Number of games = 101,052
Number of players = 1,389
Number of clusters = 1
Minimum player score = 25.0%
Minimum Elo = 2450
Minimum occurrences each player = 20
Minimum plies each game = 51
Beginning date of games = 2006.01.01
Ending date of games = 2016.02.01
Version = 2016.02.01

=================
"cc" databases
=================

High quality databases of selected medium time-control games between 
top computer chess engines. There are 2 files: cc01.pgn and cc02.pgn. 

Filtering was then done to ensure the engines/games have the 
following characteristics:

1. 64-bit 4CPU engines 
2. long time control
3. high rated engines
4. no games with the same moves
5. no games between different versions or derivatives of the 
   same engine
   
Note: Most computer engine games are "in book" for at least the 
first 8 moves.  

-------------

(1) cc01.pgn

Games from CCRL 40/40

"ECO" codes were reclassified using extended "SCID" ECO codes. 

"Elo" values were recalculated using "EloStat 1.3" with starting
value = 3000. 

Games have full-move formatting on each line.

Games were sorted in the following order: ECO, WhitePlayer, 
BlackPlayer, Result, Date.

Number of games = 32,407
Number of engines = 86
Number of clusters = 1
Minimum player score = 30.0%
Minimum Elo (recalculated) = 2900
Minimum occurrences each engine = 100
Minimum plies each game = 61
Beginning date of games = 2011.01.01
Ending date of games = 2016.02.06
Version = 2016.02.07

-------------

(2) cc02.pgn

Games from CEGT 40/20

"ECO" codes were reclassified using extended "SCID" ECO codes. 

"Elo" values were recalculated using "EloStat 1.3" with starting
value = 3000. 

Games have full-move formatting on each line.

Games were sorted in the following order: ECO, WhitePlayer, 
BlackPlayer, Result, Date.

Number of games = 37,255
Number of engines = 55
Number of clusters = 1
Minimum player score = 30.0%
Minimum Elo (recalculated) = 2900
Minimum occurrences each engine = 100
Minimum plies each game = 61
Beginning date of games = 2011.01.01
Ending date of games = 2016.02.06
Version = 2016.02.07

=================

Latest versions are at:

  http://www.hoflink.com/~npollock/chess.html


