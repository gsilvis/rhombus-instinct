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
    fn rotate_right(&mut self) {
        *self = match *self {
            Orientation::Start => Orientation::Right,
            Orientation::Right => Orientation::Start,
            Orientation::Both => Orientation::Left,
            Orientation::Left => Orientation::Both,
        }
    }
    fn rotate_left(&mut self) {
        *self = match *self {
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

const START_POSITION: Position = (4, 20);

trait Randomizer {
    fn get_piece(&mut self) -> Tetrimino;
}

#[derive(Debug)]
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

#[derive(Debug)]
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
                ReducedOrientation::Flipped => [(0, -1), (0, 0), (1, 0), (1, 1)],
            },
            Tetrimino::Z => match reduce_orientation(self.orientation) {
                ReducedOrientation::Start => [(-1, 0), (0, 0), (0, -1), (1, -1)],
                ReducedOrientation::Flipped => [(0, -1), (0, 0), (-1, 0), (-1, 1)],
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

#[derive(Debug)]
struct BoardState {
    board: Board,
    current: TetriminoState,
}

impl BoardState {
    fn occupied(&self, pos: Position) -> bool {
        let (x, y) = pos;
        if x < 0 || x >= 10 {
            true
        } else if y < 0 || y >= 21 {
            true
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
            .any(|pos| self.occupied(*pos));
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
    fn new() -> Self {
        BoardState {
            current: TetriminoState {
                tetrimino: Tetrimino::I,
                orientation: Orientation::Start,
                position: (100, 100),   // off the board.
            },
            board: [[None; 21]; 10],
        }
    }
    fn stuck(&mut self) -> bool {
        self.current.position.1 -= 1;
        let result = self.current_piece_conflicts();
        self.current.position.1 += 1;
        return result;
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
    fn clear(&mut self) -> usize {
        let mut cursor: usize = 0;
        let mut result = 0;
        for read in 0..21 {
            if (0..10).any(|x| self.board[x][read] == None) {
                for x in 0..10 {
                    self.board[x][cursor] = self.board[x][read];
                }
                cursor += 1;
                result += 1;
            }
        }
        for write in cursor..21 {
            for x in 0..10 {
                self.board[x][write] = None;
            }
        }
        return result;
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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct KeyState {
    // TODO: handle quick taps, etc.
    left: Option<usize>,  // frame count
    right: Option<usize>,
    fast_drop: Option<usize>,  // Lock if fallen; otherwise drop a frame
    sonic_drop: bool,  // Drop all the way but do not lock
    r_left: bool,
    r_right: bool,
}

impl KeyState {
    fn new() -> KeyState {
        KeyState {
            left: None, right: None,
            fast_drop: None, sonic_drop: false,
            r_left: false, r_right: false,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum State {
    Start,
    Falling,
    Are,
    Clear,
    Dead,
}

#[derive(Debug)]
struct Game {
    board: BoardState,
    rand: TGMRandomizer,  // todo parameterize.
    keys: KeyState,
    state: State,
    frames: usize,
    stuck_frames: usize,
    next: Tetrimino,
}

const LINE_CLEAR_FRAMES: usize = 41;
const ARE_FRAMES: usize = 15;
const GRAVITY_FRAMES: usize = 1;
const LOCK_FRAMES: usize = 20;

impl Game {
    fn new() -> Game {
        Game {
            board: BoardState::new(),
            rand: TGMRandomizer::new(),
            keys: KeyState::new(),
            state: State::Start,
            frames: 0,
            stuck_frames: 0,
            next: Tetrimino::I,   // doesn't matter.
        }
    }
    fn update(&mut self) {
        // Update state, and fall/lock as appropriate.
        match self.state {
            State::Dead => {
                return;
            }
            State::Start => {
                self.state = State::Are;
                self.frames = 0;
                self.stuck_frames = 0;
                self.next = self.rand.get_piece();
            },
            State::Clear => {
                if self.frames >= LINE_CLEAR_FRAMES {
                    self.state = State::Are;
                    self.frames = 0;
                } else {
                    self.frames += 1;
                }
            },
            State::Are => {
                if self.frames >= ARE_FRAMES {
                    self.board.spawn(TetriminoState {
                        tetrimino: self.next,
                        orientation: Orientation::Start,
                        position: START_POSITION,
                    });
                    if self.board.current_piece_conflicts() {
                        self.state = State::Dead;
                        return;
                    }
                    self.next = self.rand.get_piece();
                    self.state = State::Falling;
                    self.stuck_frames = 0;
                    self.frames = 0;
                } else {
                    self.frames += 1;
                }
            },
            State::Falling => {
                if self.board.stuck() {
                    self.stuck_frames += 1;
                } else {
                    self.stuck_frames = 0;
                }
                if self.stuck_frames >= LOCK_FRAMES {
                    self.board.lock();
                    if self.board.clear() > 0 {
                        self.state = State::Clear;
                    } else {
                        self.state = State::Are;
                    }
                    self.frames = 0;
                } else if self.frames >= GRAVITY_FRAMES {  // TODO: variable gravity
                    self.board.fall();
                    self.frames = 0;
                } else {
                    self.frames += 1;
                }
            },
        }

        // Handle held keys.
        if let Some(n) = self.keys.left {
            self.keys.left = Some(n+1);
        }
        if let Some(n) = self.keys.right {
            self.keys.right = Some(n+1);
        }
        if let Some(n) = self.keys.fast_drop {
            self.keys.fast_drop = Some(n+1);
        }

        if self.state != State::Falling {
            return;
        }

        // Handle input.
        // TODO: do multiple things in the right order.
        if let Some(n) = self.keys.left {
            if n == 1 || n > 14 {
                self.board.shift_left();
            }
        }
        if let Some(n) = self.keys.right {
            if n == 1 || n > 14 {
                self.board.shift_right();
            }
        }
        if self.keys.r_left {
            self.keys.r_left = false;
            self.board.rotate_left();
        }
        if self.keys.r_right {
            self.keys.r_right = false;
            self.board.rotate_right();
        }
        if self.keys.sonic_drop {
            self.keys.sonic_drop = false;
            while self.board.fall() { }
        }
        if let Some(n) = self.keys.fast_drop {
            if n == 1 || n > 14 {
                if !self.board.fall() {
                    self.board.lock();
                    if self.board.clear() > 0 {
                        self.state = State::Clear;
                    } else {
                        self.state = State::Are;
                    }
                    self.frames = 0;
                }
            }
        }
    }

    fn input(&mut self, button: &piston::input::keyboard::Key, press: bool) {
        use piston::input::keyboard::Key;
        match button {
            Key::Left => if press {
                self.keys.left = Some(0);
            } else {  //if 0 != self.keys.left.unwrap_or(0) {
                self.keys.left = None;
            },
            Key::Right => if press {
                self.keys.right = Some(0);
            } else {  //if 0 != self.keys.right.unwrap_or(0) {
                self.keys.right = None;
            },
            Key::Up => if press { self.keys.sonic_drop = true; },
            Key::Down => if press {
                self.keys.fast_drop = Some(0);
            } else { //if 0 != self.keys.fast_drop.unwrap_or(0) {
                self.keys.fast_drop = None;
            },
            Key::Z => if press { self.keys.r_left = true; },
            Key::X => if press { self.keys.r_right = true; },
            _ => {},
            // TODO handle double-rotation in a consistent manner?
        }
    }
}

fn tetrimino_color(tet: Tetrimino) -> [f32; 4] {
  match tet {
    Tetrimino::O => [1.0, 1.0, 0.0, 1.0],
    Tetrimino::I => [0.0, 1.0, 1.0, 1.0],
    Tetrimino::S => [0.0, 1.0, 0.0, 1.0],
    Tetrimino::Z => [1.0, 0.0, 0.0, 1.0],
    Tetrimino::T => [1.0, 0.0, 1.0, 1.0],
    Tetrimino::L => [1.0, 0.5, 0.0, 1.0],
    Tetrimino::J => [0.0, 0.0, 1.0, 1.0],
  }
}

fn render(game: &Game, gl: &mut opengl_graphics::GlGraphics, args: &piston::input::RenderArgs) {
    use graphics::*;

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    const GRAY:  [f32; 4] = [0.7, 0.7, 0.7, 1.0];

    // Play-field is [0, 24] [13, 24] [10, 0] [23, 0]

    let scale = args.width.min(args.height) / 280.0;

    let rhombus = [
        [0.0, 0.0], [13.0, 0.0], [18.0, -12.0], [5.0, -12.0]
    ];

    gl.draw(args.viewport(), |c, gl| {
        clear(BLACK, gl);

        let mut draw_rhomb = |(x, y): Position, color: [f32; 4]| {
            let transform = c.transform
              .scale(scale, scale)
              .trans(20.0, 260.0)
              .trans(13.0*(x as f64), 0.0)
              .trans(5.0*(y as f64), -12.0*(y as f64));
            polygon(color, &rhombus, transform, gl);
        };
        for y in -1..22 {
            draw_rhomb((-1, y), GRAY);
            draw_rhomb((10, y), GRAY);
        }
        for x in 0..10 {
            draw_rhomb((x, -1), GRAY);
            draw_rhomb((x, 21), GRAY);
        }

        let mut draw_tetrimino_rhomb = |pos: Position, tet: Tetrimino, active:bool| {
            let mut color = tetrimino_color(tet);
            if !active {
                color[3] = 0.5;
            }
            draw_rhomb(pos, color);
        };
        for x in 0..10 {
            for y in 0..21 {
                if let Some(tet) = game.board.board[x][y] {
                    draw_tetrimino_rhomb((x as i8, y as i8), tet, false);
                }
            }
        }

        let mut draw_tetrimino = |state: &TetriminoState| {
            for pos in state.occupied_places().iter() {
                draw_tetrimino_rhomb(*pos, state.tetrimino, true);
            }
        };
        if game.state == State::Falling || game.state == State::Dead {
            draw_tetrimino(&game.board.current);
        }
        if game.state != State::Dead {
            draw_tetrimino(&TetriminoState {
                tetrimino: game.next,
                position: (-4, 16),
                orientation: Orientation::Start,
            });
        }

        // let transform = c.transform.trans(x, y);

        // Draw a box rotating around the middle of the screen.
        // polygon(RED, &rhombus, transform, gl);
        // polygon(RED, &rhombus, c.transform.trans(x+65.0, y), gl);
        // polygon(GREEN, &rhombus, c.transform.trans(x-65.0, y), gl);
    });
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = opengl_graphics::OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: glutin_window::GlutinWindow = piston::window::WindowSettings::new(
            "rhombus-instinct",
            [500, 800]
        )
        .opengl(opengl)
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();


    // Create a new game and run it.
    let mut gl = opengl_graphics::GlGraphics::new(opengl);
    let mut game = Game::new();
    use piston::event_loop::EventLoop;

    let mut settings = piston::event_loop::EventSettings::new();
    settings.set_max_fps(60);
    settings.set_ups(0);

    let mut events = piston::event_loop::Events::new(settings);
    while let Some(e) = events.next(&mut window) {
        match e {
            piston::input::Event::Loop(piston::input::Loop::Render(r)) => {
                game.update();
                render(&game, &mut gl, &r);
            },
            piston::input::Event::Input(piston::input::Input::Button(args)) => {
                // println!("{:?}", args);
                if let piston::input::Button::Keyboard(key) = args.button {
                  game.input(&key, args.state == piston::input::ButtonState::Press);
                }
            },
            _ => {},
        };
    }
}