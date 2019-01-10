use rand::Rng;

// The board's bottom slants upper right to lower left.  So, the S and L have
// obtuse angles, while the Z and J have acute angles.

// I
// I OO  SS ZZ  LLL JJJ TTT
// I OO SS   ZZ L     J  T
// I

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tetrimino {
    I,
    O,
    S,
    Z,
    L,
    J,
    T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    Start,
    Right,
    Both,
    Left,
}

impl Orientation {
    fn rotate_right(&self) -> Self {
        match self {
            Orientation::Start => Orientation::Right,
            Orientation::Right => Orientation::Start,
            Orientation::Both => Orientation::Left,
            Orientation::Left => Orientation::Both,
        }
    }
    fn rotate_left(&self) -> Self {
        match self {
            Orientation::Start => Orientation::Left,
            Orientation::Right => Orientation::Both,
            Orientation::Both => Orientation::Right,
            Orientation::Left => Orientation::Start,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReducedOrientation {
    Start,
    Flipped,
}

// The 'O' piece has one orientation.  Its bounding box has no center; its
// location is defined by the '*', in the upper-left hand corner.

//   *O
//   OO

// The 'I', 'S', and 'Z' pieces have two orientations: one for Start/Both, one
// for Left/Right.  The 'I' piece's bounding box has no center; its location is
// again defined by the '*', even when it is outside the piece.

//   ....  ..I.
//   I*II  .*I.
//   ....  ..I.
//   ....  ..I.

//   ...  ..S
//   .SS  .SS
//   SS.  .S.

//   ...  Z..
//   ZZ.  ZZ.
//   .ZZ  .Z.

// The 'L', 'J', and 'T' pieces have four orientations, ordered here as Start,
// Right, Both, Left.

//   ...  .L.  ...  .LL
//   LLL  .L.  ..L  .L.
//   L..  LL.  LLL  .L.

//   ...  JJ.  ...  .J.
//   JJJ  .J.  J..  .J.
//   ..J  .J.  JJJ  .JJ

//   ...  .T.  ...  .T.
//   TTT  TT.  .T.  .TT
//   .T.  .T.  TTT  .T.

type Position = (i8, i8);

fn reduce_orientation(or: Orientation) -> ReducedOrientation {
    match or {
        Orientation::Start => ReducedOrientation::Start,
        Orientation::Both => ReducedOrientation::Start,
        Orientation::Left => ReducedOrientation::Flipped,
        Orientation::Right => ReducedOrientation::Flipped,
    }
}

const START_POSITION: Position = (4, 19);

trait Randomizer {
    fn get_piece(&mut self) -> Tetrimino;
}

struct TGMRandomizer {
    history: [Tetrimino; 4],
    pieces_given: usize,
}

impl TGMRandomizer {
    fn new() -> Self {
        TGMRandomizer {
            history: [Tetrimino::Z, Tetrimino::Z, Tetrimino::S, Tetrimino::S],
            pieces_given: 0,
        }
    }
    fn helper(&self) -> Tetrimino {
        if self.pieces_given == 0 {
            return [Tetrimino::I, Tetrimino::T, Tetrimino::L, Tetrimino::J]
                [rand::thread_rng().gen_range(0, 4)];
        } else {
            return [
                Tetrimino::O,
                Tetrimino::I,
                Tetrimino::S,
                Tetrimino::Z,
                Tetrimino::T,
                Tetrimino::L,
                Tetrimino::J,
            ][rand::thread_rng().gen_range(0, 7)];
        }
    }
}

impl Randomizer for TGMRandomizer {
    fn get_piece(&mut self) -> Tetrimino {
        let mut res = self.helper();
        for _ in 0..6 {
            if !self.history.iter().any(|saved| *saved == res) {
                break;
            }
            res = self.helper();
        }
        self.history[self.pieces_given % 4] = res;
        self.pieces_given += 1;
        return res;
    }
}

type Board = [[Option<Tetrimino>; 21]; 10];

struct TetriminoState {
    tetrimino: Tetrimino,
    orientation: Orientation,
    position: Position,
}

impl TetriminoState {
    fn occupied_places(&self) -> [Position; 4] {
        let mut result = match self.tetrimino {
            Tetrimino::O => [(0, 0), (1, 0), (0, -1), (1, -1)],
            Tetrimino::I => match reduce_orientation(self.orientation) {
                ReducedOrientation::Start => [(2, 0), (1, 0), (0, 0), (-1, 0)],
                ReducedOrientation::Flipped => [(1, 1), (1, 0), (1, -1), (1, -2)],
            },
            Tetrimino::S => match reduce_orientation(self.orientation) {
                ReducedOrientation::Start => [(1, 0), (0, 0), (0, -1), (-1, -1)],
                ReducedOrientation::Flipped => [(0, -1), (0, 0), (0, 1), (1, 1)],
            },
            Tetrimino::Z => match reduce_orientation(self.orientation) {
                ReducedOrientation::Start => [(-1, 0), (0, 0), (0, -1), (1, -1)],
                ReducedOrientation::Flipped => [(0, -1), (0, 0), (0, 1), (-1, 1)],
            },
            Tetrimino::T => match self.orientation {
                Orientation::Start => [(0, 0), (-1, 0), (1, 0), (0, -1)],
                Orientation::Right => [(0, 0), (0, 1), (0, -1), (-1, 0)],
                Orientation::Left => [(0, 0), (0, 1), (0, -1), (1, 0)],
                Orientation::Both => [(0, 0), (-1, -1), (0, -1), (1, -1)],
            },
            Tetrimino::L => match self.orientation {
                Orientation::Start => [(0, 0), (1, 0), (-1, 0), (-1, -1)],
                Orientation::Right => [(0, 0), (0, 1), (0, -1), (-1, -1)],
                Orientation::Left => [(0, 0), (0, -1), (0, 1), (1, 1)],
                Orientation::Both => [(1, 0), (1, -1), (0, -1), (-1, -1)],
            },
            Tetrimino::J => match self.orientation {
                Orientation::Start => [(0, 0), (-1, 0), (1, 0), (1, -1)],
                Orientation::Right => [(0, 0), (0, 1), (0, -1), (1, -1)],
                Orientation::Left => [(0, 0), (0, -1), (0, 1), (-1, 1)],
                Orientation::Both => [(-1, 0), (-1, -1), (0, -1), (1, -1)],
            },
        };
        for i in 0..4 {
            result[i].0 += self.position.0;
            result[i].1 += self.position.1;
        }
        result
    }
}

struct BoardState {
    board: Board,
    current: TetriminoState,
}

impl BoardState {
    fn occupied(&self, pos: Position) -> bool {
        let (x, y) = pos;
        if x < 0 || x >= 10 {
            false
        } else if y < 0 || y >= 21 {
            false
        } else {
            self.board[x as usize][y as usize] != None
        }
    }
    fn center_column_conflicts(&self) -> bool {
        let (x, y) = self.current.position;
        self.occupied((x, y - 1)) || self.occupied((x, y)) || self.occupied((x, y + 1))
    }
    fn occupied_places(&self) -> [Position; 4] {
        self.current.occupied_places()
    }
    fn current_piece_conflicts(&self) -> bool {
        return self
            .occupied_places()
            .into_iter()
            .all(|pos| !self.occupied(*pos));
    }
    fn kick_allowed(&self) -> bool {
        let (x, y) = self.current.position;
        match self.current.tetrimino {
            Tetrimino::O => false,
            Tetrimino::I => false, // Change for TGM 3 semantics.
            Tetrimino::S => true,
            Tetrimino::Z => true,
            Tetrimino::T => match reduce_orientation(self.current.orientation) {
                ReducedOrientation::Start => true,
                ReducedOrientation::Flipped => !self.center_column_conflicts(),
            },
            Tetrimino::L => match self.current.orientation {
                Orientation::Start => true,
                Orientation::Both => true,
                Orientation::Right => self.occupied((x - 1, y - 1)),
                Orientation::Left => self.occupied((x + 1, y + 1)),
            },
            Tetrimino::J => match self.current.orientation {
                Orientation::Start => true,
                Orientation::Both => true,
                Orientation::Right => self.occupied((x - 1, y + 1)),
                Orientation::Left => self.occupied((x + 1, y - 1)),
            },
        }
    }
    fn finish_rotate(&mut self) -> bool {
        if !self.current_piece_conflicts() {
            return true;
        }
        if !self.kick_allowed() {
            return false;
        }
        if self.shift_right() {
            return true;
        }
        if self.shift_left() {
            return true;
        }
        return false;
    }
    fn new(first_piece: TetriminoState) -> Self {
        BoardState {
            current: first_piece,
            board: [[None; 21]; 10],
        }
    }
    fn fall(&mut self) -> bool {
        self.current.position.1 -= 1;
        if self.current_piece_conflicts() {
            self.current.position.1 += 1;
            return false;
        }
        return true;
    }
    fn rotate_right(&mut self) -> bool {
        self.current.orientation.rotate_right();
        if !self.finish_rotate() {
            self.current.orientation.rotate_right();
            return false;
        }
        return true;
    }
    fn rotate_left(&mut self) -> bool {
        self.current.orientation.rotate_left();
        if !self.finish_rotate() {
            self.current.orientation.rotate_left();
            return false;
        }
        return true;
    }
    fn shift_left(&mut self) -> bool {
        self.current.position.0 -= 1;
        if self.current_piece_conflicts() {
            self.current.position.0 += 1;
            return false;
        }
        return true;
    }
    fn shift_right(&mut self) -> bool {
        self.current.position.0 += 1;
        if self.current_piece_conflicts() {
            self.current.position.0 -= 1;
            return false;
        }
        return true;
    }
    fn lock(&mut self) {
        for (x, y) in self.occupied_places().into_iter() {
            self.board[(*x) as usize][(*y) as usize] = Some(self.current.tetrimino);
        }
    }
    fn clear(&mut self) {
        let mut cursor: usize = 0;
        for read in 0..21 {
            if (0..10).any(|x| self.board[x][read] == None) {
                for x in 0..10 {
                    self.board[x][cursor] = self.board[x][read];
                }
                cursor += 1;
            }
        }
        for write in cursor..21 {
            for x in 0..10 {
                self.board[x][write] = None;
            }
        }
    }
    fn spawn(&mut self, new_piece: TetriminoState) {
        self.current = new_piece;
    }
}

// Proper game logic:
// - spawn
// - current_piece_conflicts [determine game end]
// - rotate_right, rotate_left, shift_right, shift_left, fall
// - lock
// - clear

fn main() {
    println!("Hello, world!");
}
