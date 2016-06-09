extern crate regex;
use self::regex::Regex;

#[derive(PartialEq, Debug)]
pub enum Piece {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

impl Piece {
    fn to_char(&self) -> char {
        match *self {
            Piece::Rook   => 'R',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Queen  => 'Q',
            Piece::King   => 'K',
            Piece::Pawn   => 'P',
        }
    }

    fn from(c: char) -> Option<Piece> {
        match c {
            'R' => Some(Piece::Rook),
            'N' => Some(Piece::Knight),
            'B' => Some(Piece::Bishop),
            'Q' => Some(Piece::Queen),
            'K' => Some(Piece::King),
            'P' => Some(Piece::Pawn),
            _   => None,
        }
    }
}

#[test]
fn test_piece_to_str() {
    assert_eq!('R', Piece::Rook.to_char());
}

#[test]
fn test_piece_from_str() {
    assert_eq!(Some(Piece::Knight), Piece::from('N'));
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
pub enum Castle {
    Queenside,
    Kingside,
}

impl Castle {
    fn to_str(&self) -> String {
        match *self {
            Castle::Queenside => String::from("O-O-O"),
            Castle::Kingside  => String::from("O-O"),
        }
    }
    
    fn from_str(s: String) -> Option<Castle> {
        match s.as_ref() {
            "O-O"   => Some(Castle::Kingside),
            "O-O-O" => Some(Castle::Queenside),
            _       => None
        }
    }
}

#[test]
fn test_castle_to_str() {
    assert_eq!(Castle::Queenside.to_str(), "O-O-O");
}

#[test]
fn test_castle_from_str() {
    assert_eq!(Some(Castle::Queenside),
               Castle::from_str(String::from("O-O-O")));
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
struct Move {
    piece: Option<Piece>,
    from_file: Option<char>, // column, 'a' to 'h'
    from_rank: Option<i8>,   // row,     1  to  8
    capture: bool,
    to_file: Option<char>,   // column, 'a' to 'h'
    to_rank: Option<i8>,     // row,     1  to  8
    check: bool,
    checkmate: bool,
    promotion: Option<Piece>,
    castle: Option<Castle>,
}

impl Move {
    fn new() -> Move {
        Move {
            piece: None,
            from_file: None,
            from_rank: None,
            capture: false,
            to_file: None,
            to_rank: None,
            promotion: None,
            castle: None,
            check: false,
            checkmate: false,
        }
    }
    
    pub fn from(mov_str: &String) -> Move {
        // WOW, I should really write a regex for this
        let mut mov = Move::new();
        if mov_str.contains("x") {
            mov.capture = true;
        }
        if mov_str.contains("+") {
            mov.check = true;
        }
        if mov_str.contains("#") {
            mov.checkmate = true;
        }
        // Piece
        if mov_str.starts_with("R") {
            mov.piece = Some(Piece::Rook);
        } else if mov_str.starts_with("N") {
            mov.piece = Some(Piece::Knight);
        } else if mov_str.starts_with("B") {
            mov.piece = Some(Piece::Bishop);
        } else if mov_str.starts_with("Q") {
            mov.piece = Some(Piece::Queen);
        } else if mov_str.starts_with("K") {
            mov.piece = Some(Piece::King);
        } else if mov_str.contains("O-O-O") {
            // no piece
            mov.castle = Some(Castle::Queenside);
        } else if mov_str.contains("O-O") {
            // no piece
            mov.castle = Some(Castle::Kingside);
        } else {
            mov.piece = Some(Piece::Pawn);
        }
        if mov_str.contains("=") {
            // Promotion
            mov.promotion = Piece::from(mov_str.chars().nth(
                mov_str.find("=").unwrap() + 1).unwrap());
        }
        // Get from/to positions if there
        let move_re = Regex::new(r"(?P<piece>[RNBQK])?(?P<from_file>[abcdefgh])?(?P<from_rank>[12345678])?(?P<capture>x)?(?P<to_file>[abcdefgh])(?P<to_rank>[12345678])(?P<promotion>=[RNBQ])?(?P<check>\+)?(?P<checkmate>\#)?").unwrap();
        let move_caps = move_re.captures(mov_str);
        // In future, we could use the following regex for castle moves
        // let castle_re = Regex::new(r"O\-O(?P<qs>\-O)(?P<check>\+)?(?P<checkmate>\#)").unwrap();
        // let castle_caps = castle_re.captures(mov_str);
        if move_caps.is_some() {
            let c = move_caps.unwrap();
            if c.name("from_file").is_some() {
                mov.from_file =
                    c.name("from_file").unwrap().chars().nth(0);
            }
            if c.name("from_rank").is_some() {
                mov.from_rank =
                    Some(c.name("from_rank").unwrap().chars().nth(0).unwrap()
                         .to_digit(10).unwrap() as i8);
            }
            if c.name("to_file").is_some() {
                mov.to_file =
                    c.name("to_file").unwrap().chars().nth(0);
            }
            if c.name("to_rank").is_some() {
                mov.to_rank =
                    Some(c.name("to_rank").unwrap().chars().nth(0).unwrap()
                         .to_digit(10).unwrap() as i8);
            }
        }
        mov
    }

    pub fn to_str(&self) -> String {
        let mut ret = String::new();
        match self.piece {
            Some(Piece::Pawn) => (),
            Some(ref p) => ret.push(p.to_char()),
            _ => (),
        }
        if self.from_file.is_some() {
            ret.push(self.from_file.unwrap());
        }
        if self.from_rank.is_some() {
            ret.push(self.from_rank.unwrap().to_string().pop().unwrap());
        }
        if self.capture {
            ret.push('x');
        }
        if self.to_file.is_some() {
            ret.push(self.to_file.unwrap());
        }
        if self.to_rank.is_some() {
            ret.push(self.to_rank.unwrap().to_string().pop().unwrap());
        }
        match self.promotion {
            Some(Piece::Pawn) => (),
            Some(ref p) => {
                ret.push('=');
                ret.push(p.to_char());
            },
            _ => (),
        }
        match self.castle {
            Some(Castle::Kingside)  => ret.push_str("O-O"),
            Some(Castle::Queenside) => ret.push_str("O-O-O"),
            _ => (),
        }
        if self.check {
            ret.push('+');
        }
        if self.checkmate {
            ret.push('#');
        }
        ret
    }
}

#[test]
fn test_castle_move_from() {
    let mut m = Move::new();
    m.castle = Some(Castle::Kingside);
    assert_eq!(Move::from(&String::from("O-O")), m);
    m.checkmate = true;
    assert_eq!(Move::from(&String::from("O-O#")), m);
    m = Move::new();
    m.castle = Some(Castle::Queenside);
    m.check = true;
    assert_eq!(Move::from(&String::from("O-O-O+")), m);
}

#[test]
fn test_promotion_move_from() {
    let m = Move::from(&String::from("e8=Q"));
    let mut m2 = Move::new();
    m2.piece = Some(Piece::Pawn);
    m2.to_file = Some('e');
    m2.to_rank = Some(8 as i8);
    m2.promotion = Some(Piece::Queen);
    assert_eq!(m2, m);
}

#[test]
fn test_pawn_move_to_str() {
    let mut m = Move::new();
    m.to_file = Some('e');
    m.to_rank = Some(4 as i8);
    assert_eq!(m.to_str(), "e4");
    m.piece = Some(Piece::Pawn);
    assert_eq!(m.to_str(), "e4");
}

#[test]
fn test_longest_move_to_str() {
    // https://www.chess.com/blog/kurtgodden/think-you-know-algebraic-notation
    // according to the above site, the longest legal move is exd6+ e.p.,
    // but since PGN doesn't have special notation for en passant, I came up
    // with exd8=Q+
    let mut m = Move::new();
    m.piece = Some(Piece::Pawn);
    m.from_file = Some('e');
    m.capture = true;
    m.to_file = Some('d');
    m.to_rank = Some(8 as i8);
    m.promotion = Some(Piece::Queen);
    m.check = true;
    assert_eq!(m.to_str(), "exd8=Q+");
}

#[test]
fn test_move_to_str_and_from_str() {
    let mut m = Move::new();
    m.piece = Some(Piece::Pawn);
    m.from_file = Some('e');
    m.capture = true;
    m.to_file = Some('d');
    m.to_rank = Some(8 as i8);
    m.promotion = Some(Piece::Queen);
    m.check = true;
    assert_eq!(Move::from(&m.to_str()), m);
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
struct Posn {
    file: char, // 'a' to 'h'
    rank: i8,   //  1  to  8
}

impl Posn {
    fn from(s: &String) -> Posn {
        Posn {
            file: s.chars().nth(0).unwrap(),
            rank: s.chars().nth(1).unwrap().to_digit(10).unwrap() as i8,
        }
    }
}

#[test]
fn test_posn_from() {
    assert_eq!(Posn { file: 'e', rank: 4 as i8 },
               Posn::from(&String::from("e4")));
}

pub enum Player {
    White,
    Black,
}

#[derive(PartialEq, Debug)]
struct State {
    board: [[char; 8]; 8]
}

impl State {
    pub fn new() -> State {
        State {
            //        a    b    c    d    e    f    g    h
            board: [['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'], // 8
                    ['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'], // 7
                    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 6
                    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 5
                    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 4
                    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 3
                    ['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'], // 2
                    ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖']],// 1
        }
    }

    fn col_from_file(f: &char) -> Option<usize> {
        match f {
            &'a' => Some(0 as usize),
            &'b' => Some(1 as usize),
            &'c' => Some(2 as usize),
            &'d' => Some(3 as usize),
            &'e' => Some(4 as usize),
            &'f' => Some(5 as usize),
            &'g' => Some(6 as usize),
            &'h' => Some(7 as usize),
             _  => None,
        }
    }

    fn row_from_rank(r: &i8) -> Option<usize> {
        match r {
            &1 => Some(7 as usize),
            &2 => Some(6 as usize),
            &3 => Some(5 as usize),
            &4 => Some(4 as usize),
            &5 => Some(3 as usize),
            &6 => Some(2 as usize),
            &7 => Some(1 as usize),
            &8 => Some(0 as usize),
            _ => None,
        }
    }

    fn find_all_pieces(&self, player: Player, piece: Piece) -> Vec<Posn> {
        let v = Vec::new();
        v
    }

    fn find_piece_location(&self, player: Player, mov: Move) -> Option<Posn> {
        // TODO
        None
    }

    pub fn make_move(&self, player: Player, mov: Move) -> Option<State> {
        let to = Posn { file: mov.to_file.unwrap(),
                        rank: mov.to_rank.unwrap(), };
        match self.find_piece_location(player, mov) {
            Some(from) => Some(self.move_piece(from, to)),
            _ => None,
        }
    }

    fn move_piece(&self, from: Posn, to: Posn) -> State {
        let mut clone = self.clone();
        clone.board[State::row_from_rank(&to.rank).unwrap()]
            [State::col_from_file(&to.file).unwrap()] = 
        clone.board[State::row_from_rank(&from.rank).unwrap()]
            [State::col_from_file(&from.file).unwrap()];
        clone.board[State::row_from_rank(&from.rank).unwrap()]
            [State::col_from_file(&from.file).unwrap()] = ' ';
        clone
    }
}

#[test]
fn test_col_from_file() {
    assert_eq!(Some(4 as usize), State::col_from_file(&'e'));
}

#[test]
fn test_row_from_rank() {
    assert_eq!(Some(4 as usize), State::row_from_rank(&(4 as i8)));
}

#[test]
fn test_move_piece() {
    assert_eq!(State::new().move_piece(Posn::from(&String::from("e2")),
                                       Posn::from(&String::from("e4"))),
               State {
                   //        a    b    c    d    e    f    g    h
                   board: [['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'], // 8
                           ['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'], // 7
                           [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 6
                           [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 5
                           [' ', ' ', ' ', ' ', '♙', ' ', ' ', ' '], // 4
                           [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // 3
                           ['♙', '♙', '♙', '♙', ' ', '♙', '♙', '♙'], // 2
                           ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖']],// 1
               });
}

impl Clone for State {
    fn clone(&self) -> State {
        State { board: self.board.clone(), }
    }
}
