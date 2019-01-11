use rand::Rng;

// The board's bottom slants upper right to lower left.  So, the S and L have
// obtuse angles, while the Z and J have acute angles.

// I
// I OO  SS ZZ  LLL JJJ TTT
// I OO SS   ZZ L     J  T
// I

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tetrhombino {
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
            Orientation::Right => Orientation::Both,
            Orientation::Both => Orientation::Left,
            Orientation::Left => Orientation::Start,
        }
    }
    fn rotate_left(&mut self) {
        *self = match *self {
            Orientation::Start => Orientation::Left,
            Orientation::Left => Orientation::Both,
            Orientation::Both => Orientation::Right,
            Orientation::Right => Orientation::Start,
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

//   ...  S..
//   .SS  SS.
//   SS.  .S.

//   ...  ..Z
//   ZZ.  .ZZ
//   .ZZ  .Z.

// The 'L', 'J', and 'T' pieces have four orientations, ordered here as Start,
// Right, Both, Left.

//   ...  LL.  ...  .L.
//   LLL  .L.  ..L  .L.
//   L..  .L.  LLL  .LL

//   ...   .J. ...  .JJ
//   JJJ   .J. J..  .J.
//   ..J   JJ. JJJ  .J.

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
    fn get_piece(&mut self) -> Tetrhombino;
}

#[derive(Debug)]
struct TGMRandomizer {
    history: [Tetrhombino; 4],
    pieces_given: usize,
}

impl TGMRandomizer {
    fn new() -> Self {
        TGMRandomizer {
            history: [Tetrhombino::Z, Tetrhombino::Z, Tetrhombino::S, Tetrhombino::S],
            pieces_given: 0,
        }
    }
    fn helper(&self) -> Tetrhombino {
        if self.pieces_given == 0 {
            return [Tetrhombino::I, Tetrhombino::T, Tetrhombino::L, Tetrhombino::J]
                [rand::thread_rng().gen_range(0, 4)];
        } else {
            return [
                Tetrhombino::O,
                Tetrhombino::I,
                Tetrhombino::S,
                Tetrhombino::Z,
                Tetrhombino::T,
                Tetrhombino::L,
                Tetrhombino::J,
            ][rand::thread_rng().gen_range(0, 7)];
        }
    }
}

impl Randomizer for TGMRandomizer {
    fn get_piece(&mut self) -> Tetrhombino {
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

type Board = [[Option<Tetrhombino>; 22]; 10];

#[derive(Debug)]
struct TetrhombinoState {
    tetrhombino: Tetrhombino,
    orientation: Orientation,
    position: Position,
}

impl TetrhombinoState {
    fn occupied_places(&self) -> [Position; 4] {
        let mut result = match self.tetrhombino {
            Tetrhombino::O => [(0, 0), (1, 0), (0, -1), (1, -1)],
            Tetrhombino::I => match reduce_orientation(self.orientation) {
                ReducedOrientation::Start => [(2, 0), (1, 0), (0, 0), (-1, 0)],
                ReducedOrientation::Flipped => [(1, 1), (1, 0), (1, -1), (1, -2)],
            },
            Tetrhombino::S => match reduce_orientation(self.orientation) {
                ReducedOrientation::Start => [(1, 0), (0, 0), (0, -1), (-1, -1)],
                ReducedOrientation::Flipped => [(0, -1), (0, 0), (-1, 0), (-1, 1)],
            },
            Tetrhombino::Z => match reduce_orientation(self.orientation) {
                ReducedOrientation::Start => [(-1, 0), (0, 0), (0, -1), (1, -1)],
                ReducedOrientation::Flipped => [(0, -1), (0, 0), (1, 0), (1, 1)],
            },
            Tetrhombino::T => match self.orientation {
                Orientation::Start => [(0, 0), (-1, 0), (1, 0), (0, -1)],
                Orientation::Right => [(0, 0), (0, 1), (0, -1), (-1, 0)],
                Orientation::Left => [(0, 0), (0, 1), (0, -1), (1, 0)],
                Orientation::Both => [(0, 0), (-1, -1), (0, -1), (1, -1)],
            },
            Tetrhombino::L => match self.orientation {
                Orientation::Start => [(0, 0), (1, 0), (-1, 0), (-1, -1)],
                Orientation::Right => [(0, 0), (0, 1), (0, -1), (1, -1)],
                Orientation::Left => [(0, 0), (0, -1), (0, 1), (-1, 1)],
                Orientation::Both => [(1, 0), (1, -1), (0, -1), (-1, -1)],
            },
            Tetrhombino::J => match self.orientation {
                Orientation::Start => [(0, 0), (-1, 0), (1, 0), (1, -1)],
                Orientation::Right => [(0, 0), (0, 1), (0, -1), (-1, -1)],
                Orientation::Left => [(0, 0), (0, -1), (0, 1), (1, 1)],
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
    current: TetrhombinoState,
}

impl BoardState {
    fn occupied(&self, pos: Position) -> bool {
        let (x, y) = pos;
        if x < 0 || x >= 10 {
            true
        } else if y < 0 || y >= 22 {
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
        match self.current.tetrhombino {
            Tetrhombino::O => false,
            Tetrhombino::I => false, // Change for TGM 3 semantics.
            Tetrhombino::S => true,
            Tetrhombino::Z => true,
            Tetrhombino::T => match reduce_orientation(self.current.orientation) {
                ReducedOrientation::Start => true,
                ReducedOrientation::Flipped => !self.center_column_conflicts(),
            },
            Tetrhombino::L => match self.current.orientation {
                Orientation::Start => true,
                Orientation::Both => true,
                Orientation::Right => self.occupied((x - 1, y - 1)),
                Orientation::Left => self.occupied((x + 1, y + 1)),
            },
            Tetrhombino::J => match self.current.orientation {
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
            current: TetrhombinoState {
                tetrhombino: Tetrhombino::I,
                orientation: Orientation::Start,
                position: (100, 100), // off the board.
            },
            board: [[None; 22]; 10],
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
            self.board[(*x) as usize][(*y) as usize] = Some(self.current.tetrhombino);
        }
    }
    fn clear(&mut self) -> usize {
        let mut cursor: usize = 0;
        let mut result = 0;
        for read in 0..22 {
            if (0..10).any(|x| self.board[x][read] == None) {
                for x in 0..10 {
                    self.board[x][cursor] = self.board[x][read];
                }
                cursor += 1;
                result += 1;
            }
        }
        for write in cursor..22 {
            for x in 0..10 {
                self.board[x][write] = None;
            }
        }
        return result;
    }
    fn spawn(&mut self, new_piece: TetrhombinoState) {
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
struct SingleKey {
    triggered: bool,
    needs_service: bool,
}

impl SingleKey {
    fn new() -> Self {
        SingleKey {
            triggered: false,
            needs_service: false,
        }
    }
    fn trigger(&mut self, press: bool) {
        if press {
            if !self.triggered {
                self.triggered = true;
                self.needs_service = true;
            }
        } else {
            self.triggered = false;
        }
    }
    fn service(&mut self) -> bool {
        self.triggered && std::mem::replace(&mut self.needs_service, false)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct MultiKey {
    state: SingleKey,
    hold_frames: usize,
}

impl MultiKey {
    fn new() -> Self {
        MultiKey {
            state: SingleKey::new(),
            hold_frames: 0,
        }
    }
    fn trigger(&mut self, press: bool) {
        self.state.trigger(press);
        if !press {
            self.hold_frames = 0;
        }
    }
    fn service(&mut self) -> bool {
        if self.state.triggered {
            self.hold_frames += 1;
            self.state.service() || self.hold_frames >= 14
        } else {
            false
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct ContinuousKey {
    pressed: bool,
}

impl ContinuousKey {
    fn new() -> Self {
        ContinuousKey { pressed: false }
    }
    fn trigger(&mut self, press: bool) {
        self.pressed = press;
    }
    fn service(&mut self) -> bool {
        self.pressed
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct KeyState {
    left: MultiKey,
    right: MultiKey,
    sonic_drop: MultiKey,     // Drop all the way but do not lock
    fast_drop: ContinuousKey, // Lock if fallen; otherwise drop a frame
    r_left: SingleKey,
    r_right: SingleKey,
}

impl KeyState {
    fn new() -> KeyState {
        KeyState {
            left: MultiKey::new(),
            right: MultiKey::new(),
            fast_drop: ContinuousKey::new(),
            sonic_drop: MultiKey::new(),
            r_left: SingleKey::new(),
            r_right: SingleKey::new(),
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
    rand: TGMRandomizer, // todo parameterize.
    keys: KeyState,
    state: State,
    frames: usize,
    stuck_frames: usize,
    next: Tetrhombino,
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
            next: Tetrhombino::I, // doesn't matter.
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
            }
            State::Clear => {
                if self.frames >= LINE_CLEAR_FRAMES {
                    self.state = State::Are;
                    self.frames = 0;
                } else {
                    self.frames += 1;
                }
            }
            State::Are => {
                if self.frames >= ARE_FRAMES {
                    self.board.spawn(TetrhombinoState {
                        tetrhombino: self.next,
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
            }
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
                } else if self.frames >= GRAVITY_FRAMES {
                    // TODO: variable gravity
                    self.board.fall();
                    self.frames = 0;
                } else {
                    self.frames += 1;
                }
            }
        }

        if self.state != State::Falling {
            // Handle DAS during ARE.
            self.keys.left.service();
            self.keys.right.service();
            return;
        }

        // Handle input.
        // TODO: do multiple things in the right order.
        if self.keys.left.service() {
            self.board.shift_left();
        }
        if self.keys.right.service() {
            self.board.shift_right();
        }
        if self.keys.r_left.service() {
            self.board.rotate_left();
        }
        if self.keys.r_right.service() {
            self.board.rotate_right();
        }
        if self.keys.sonic_drop.service() {
            while self.board.fall() {}
        }
        if self.keys.fast_drop.service() {
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

    fn input(&mut self, button: &piston::input::keyboard::Key, press: bool) {
        use piston::input::keyboard::Key;
        match button {
            Key::Left => self.keys.left.trigger(press),
            Key::Right => self.keys.right.trigger(press),
            Key::Up => self.keys.sonic_drop.trigger(press),
            Key::Down => self.keys.fast_drop.trigger(press),
            Key::Z => self.keys.r_left.trigger(press),
            Key::X => self.keys.r_right.trigger(press),
            _ => {}
            // TODO handle double-rotation in a consistent manner?
        }
    }
}

fn tetrhombino_color(tet: Tetrhombino) -> [f32; 4] {
    match tet {
        Tetrhombino::O => [1.0, 1.0, 0.0, 1.0],
        Tetrhombino::I => [0.0, 1.0, 1.0, 1.0],
        Tetrhombino::S => [0.0, 1.0, 0.0, 1.0],
        Tetrhombino::Z => [1.0, 0.0, 0.0, 1.0],
        Tetrhombino::T => [1.0, 0.0, 1.0, 1.0],
        Tetrhombino::L => [1.0, 0.5, 0.0, 1.0],
        Tetrhombino::J => [0.0, 0.0, 1.0, 1.0],
    }
}

fn render(game: &Game, gl: &mut opengl_graphics::GlGraphics, args: &piston::input::RenderArgs) {
    use graphics::*;

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    const GRAY: [f32; 4] = [0.7, 0.7, 0.7, 1.0];

    // Play-field is [0, 24] [13, 24] [10, 0] [23, 0]

    let scale = args.width.min(args.height) / 290.0;

    let rhombus = [[0.0, 0.0], [13.0, 0.0], [18.0, -12.0], [5.0, -12.0]];

    gl.draw(args.viewport(), |c, gl| {
        clear(BLACK, gl);

        let mut draw_rhomb = |(x, y): Position, color: [f32; 4]| {
            let transform = c
                .transform
                .scale(scale, scale)
                .trans(20.0, 280.0)
                .trans(13.0 * (x as f64), 0.0)
                .trans(5.0 * (y as f64), -12.0 * (y as f64));
            polygon(color, &rhombus, transform, gl);
        };
        for y in -1..23 {
            draw_rhomb((-1, y), GRAY);
            draw_rhomb((10, y), GRAY);
        }
        for x in 0..10 {
            draw_rhomb((x, -1), GRAY);
            draw_rhomb((x, 22), GRAY);
        }

        let mut draw_tetrhombino_rhomb = |pos: Position, tet: Tetrhombino, active: bool| {
            let mut color = tetrhombino_color(tet);
            if !active {
                color[3] = 0.5;
            }
            draw_rhomb(pos, color);
        };
        for x in 0..10 {
            for y in 0..22 {
                if let Some(tet) = game.board.board[x][y] {
                    draw_tetrhombino_rhomb((x as i8, y as i8), tet, false);
                }
            }
        }

        let mut draw_tetrhombino = |state: &TetrhombinoState| {
            for pos in state.occupied_places().iter() {
                draw_tetrhombino_rhomb(*pos, state.tetrhombino, true);
            }
        };
        if game.state == State::Falling || game.state == State::Dead {
            draw_tetrhombino(&game.board.current);
        }
        if game.state != State::Dead {
            draw_tetrhombino(&TetrhombinoState {
                tetrhombino: game.next,
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
    let mut window: glutin_window::GlutinWindow =
        piston::window::WindowSettings::new("rhombus-instinct", [500, 800])
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
            }
            piston::input::Event::Input(piston::input::Input::Button(args)) => {
                // println!("{:?}", args);
                if let piston::input::Button::Keyboard(key) = args.button {
                    game.input(&key, args.state == piston::input::ButtonState::Press);
                }
            }
            _ => {}
        };
    }
}
