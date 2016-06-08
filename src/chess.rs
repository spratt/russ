pub enum Player {
    White,
    Black,
}

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

    fn from_char(c: char) -> Option<Piece> {
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
    assert_eq!(Some(Piece::Knight), Piece::from_char('N'));
}

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

struct Move {
    piece: Option<Piece>,
    from_file: Option<char>, // column, 'a' to 'h'
    from_rank: Option<i8>,   // row,     1  to  8
    capture: bool,
    to_file: Option<char>,   // column, 'a' to 'h'
    to_rank: Option<i8>,     // row,     1  to  8
    en_passant: bool,
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
            en_passant: false,
        }
    }
    
    pub fn from(mov_str: &String) -> Move {
        let mut mov = Move::new();
        if mov_str.contains("x") {
            mov.capture = true;
        }
        if mov_str.contains("=") {
            // Promotion
            // TODO
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
        if self.en_passant {
            ret.push_str(" e.p.");
        }
        ret
    }
}

#[test]
fn test_pawn_move() {
    let mut m = Move::new();
    m.to_file = Some('e');
    m.to_rank = Some(4 as i8);
    assert_eq!(m.to_str(), "e4");
    m.piece = Some(Piece::Pawn);
    assert_eq!(m.to_str(), "e4");
}

#[test]
fn test_longest_move() {
    let mut m = Move::new();
    m.from_file = Some('e');
    m.capture = true;
    m.to_file = Some('d');
    m.to_rank = Some(6 as i8);
    m.check = true;
    m.en_passant = true;
    assert_eq!(m.to_str(), "exd6+ e.p.");
}

struct State {
    board: [[char; 8]; 8]
}

impl State {
    pub fn new() -> State {
        State {
            board: [['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'],
                    ['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'],
                    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                    ['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'],
                    ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖']],
        }
    }

    pub fn make_move(&self, player: Player, mov: Move) {
    }
}