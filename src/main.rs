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
            history: [
                Tetrhombino::Z,
                Tetrhombino::Z,
                Tetrhombino::S,
                Tetrhombino::S,
            ],
            pieces_given: 0,
        }
    }
    fn helper(&self) -> Tetrhombino {
        if self.pieces_given == 0 {
            return [
                Tetrhombino::I,
                Tetrhombino::T,
                Tetrhombino::L,
                Tetrhombino::J,
            ][rand::thread_rng().gen_range(0, 4)];
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
                ReducedOrientation::Flipped => [(0, -1), (0, 0), (1, 0), (1, 1)],
            },
            Tetrhombino::Z => match reduce_orientation(self.orientation) {
                ReducedOrientation::Start => [(-1, 0), (0, 0), (0, -1), (1, -1)],
                ReducedOrientation::Flipped => [(0, -1), (0, 0), (-1, 0), (-1, 1)],
            },
            Tetrhombino::T => match self.orientation {
                Orientation::Start => [(0, 0), (-1, 0), (1, 0), (0, -1)],
                Orientation::Right => [(0, 0), (0, 1), (0, -1), (-1, 0)],
                Orientation::Left => [(0, 0), (0, 1), (0, -1), (1, 0)],
                Orientation::Both => [(0, 0), (-1, -1), (0, -1), (1, -1)],
            },
            Tetrhombino::L => match self.orientation {
                Orientation::Start => [(0, 0), (1, 0), (-1, 0), (-1, -1)],
                Orientation::Right => [(0, 0), (0, 1), (0, -1), (-1, -1)],
                Orientation::Left => [(0, 0), (0, -1), (0, 1), (1, 1)],
                Orientation::Both => [(1, 0), (1, -1), (0, -1), (-1, -1)],
            },
            Tetrhombino::J => match self.orientation {
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
            } else {
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

trait DifficultyCurve {
    fn get_gravity(&self) -> usize; // units are G/256
    fn get_are_frames(&self) -> usize;
    fn get_clear_frames(&self) -> usize;
    fn get_lock_frames(&self) -> usize;
    fn clear_lines(&mut self, lines: usize);
    fn done(&self) -> bool;
}

#[derive(Debug)]
struct NormalDifficulty {
    lines_cleared: usize,
}

impl NormalDifficulty {
    fn new() -> Self {
        NormalDifficulty { lines_cleared: 0 }
    }
}

impl DifficultyCurve for NormalDifficulty {
    fn get_gravity(&self) -> usize {
        const SPEED_TABLE: [usize; 15] =
            [4, 8, 12, 16, 20, 24, 28, 32, 48, 64, 80, 96, 112, 128, 256];
        SPEED_TABLE[self.lines_cleared / 10]
    }
    fn get_are_frames(&self) -> usize {
        25
    }
    fn get_clear_frames(&self) -> usize {
        40
    }
    fn get_lock_frames(&self) -> usize {
        30
    }
    fn clear_lines(&mut self, lines: usize) {
        self.lines_cleared += lines;
    }
    fn done(&self) -> bool {
        self.lines_cleared >= 150
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum State {
    Start,
    Falling,
    Are(usize),   // frame count
    Clear(usize), // frame count
    Loss,
    Victory,
}

#[derive(Debug)]
struct Game {
    board: BoardState,
    rand: TGMRandomizer, // todo parameterize.
    keys: KeyState,
    state: State,
    stuck_frames: usize,
    gravity_count: usize,
    next: Tetrhombino,
    lines_cleared: usize,
    stage: NormalDifficulty, // todo parameterize.
}

impl Game {
    fn new() -> Game {
        Game {
            board: BoardState::new(),
            rand: TGMRandomizer::new(),
            keys: KeyState::new(),
            state: State::Start,
            stuck_frames: 0,
            gravity_count: 0,
            next: Tetrhombino::I, // doesn't matter.
            lines_cleared: 0,
            stage: NormalDifficulty::new(),
        }
    }
    fn spawn(&mut self) {
        self.board.spawn(TetrhombinoState {
            tetrhombino: self.next,
            orientation: Orientation::Start,
            position: START_POSITION,
        });
        if self.board.current_piece_conflicts() {
            self.state = State::Loss;
            return;
        }
        self.next = self.rand.get_piece();
        self.state = State::Falling;
        self.stuck_frames = 0;
        self.gravity_count = 0;
    }
    fn lock(&mut self) {
        self.board.lock();
        let cleared = self.board.clear();
        if cleared > 0 {
            self.lines_cleared += cleared;
            self.stage.clear_lines(cleared);
            self.state = State::Clear(0);
        } else {
            self.state = State::Are(0);
        }
        if self.stage.done() {
            self.state = State::Victory;
        }
    }
    fn update(&mut self) {
        if self.state == State::Loss || self.state == State::Victory {
            return;
        }

        // Progress through inter-piece state machine; keep this in this order
        // so that 0-frame Are and Clear phases work correctly.
        if self.state == State::Start {
            self.state = State::Are(0);
            self.next = self.rand.get_piece();
        }
        if let State::Clear(n) = self.state {
            if n >= self.stage.get_clear_frames() {
                self.state = State::Are(0);
            } else {
                self.state = State::Clear(n + 1);
            }
        }
        if let State::Are(n) = self.state {
            if n >= self.stage.get_are_frames() {
                self.spawn();
            } else {
                self.state = State::Are(n + 1);
            }
        }

        // Handle DAS during ARE/Line-clear
        if self.state != State::Falling {
            self.keys.left.service();
            self.keys.right.service();
            return;
        }

        // Input
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
                self.lock();
                return;
            }
        }

        // Fall
        if !self.board.stuck() {
            self.stuck_frames = 0;
            self.gravity_count += self.stage.get_gravity();
            while self.gravity_count >= 256 {
                self.board.fall();
                self.gravity_count -= 256;
            }
        } else {
            self.stuck_frames += 1;
        }

        // Lock
        if self.board.stuck() {
            self.gravity_count = 0;
            if self.stuck_frames >= self.stage.get_lock_frames() {
                self.lock();
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
            _ => {} // TODO handle double-rotation in a consistent manner?
        }
    }

    fn draw_rhomb(
        &self,
        (x, y): Position,
        color: [f32; 4],
        ctxt: graphics::context::Context,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        graphics::Rectangle::new(color).draw(
            [x as f64, y as f64, 1.0, 1.0],
            &ctxt.draw_state,
            ctxt.transform,
            gl,
        );
    }

    fn draw_tetrhombino(
        &self,
        state: &TetrhombinoState,
        ctxt: graphics::context::Context,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        let color = tetrhombino_color(state.tetrhombino);
        for pos in state.occupied_places().iter() {
            self.draw_rhomb(*pos, color, ctxt, gl);
        }
    }

    fn draw_segment(
        &self,
        on: bool,
        ctxt: graphics::context::Context,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        const SEGMENT: [[f64; 2]; 6] = [
            [0.05, 0.0],
            [0.10, 0.05],
            [0.90, 0.05],
            [0.95, 0.0],
            [0.90, -0.05],
            [0.10, -0.05],
        ];

        const COLOR_ON: [f32; 4] = [0.0, 0.7, 1.0, 1.0];
        if on {
            graphics::polygon(COLOR_ON, &SEGMENT, ctxt.transform, gl);
        }
    }

    fn draw_digit(
        &self,
        digit: u8,
        ctxt: graphics::context::Context,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        //   4
        // 0   2
        //   5
        // 1   3
        //   6
        const DIGITS: [[bool; 7]; 10] = [
            [true, true, true, true, true, false, true],
            [false, false, true, true, false, false, false],
            [false, true, true, false, true, true, true],
            [false, false, true, true, true, true, true],
            [true, false, true, true, false, true, false],
            [true, false, false, true, true, true, true],
            [true, true, false, true, true, true, true],
            [false, false, true, true, true, false, false],
            [true, true, true, true, true, true, true],
            [true, false, true, true, true, true, true],
        ];

        use graphics::Transformed;
        let ctxts = [
            ctxt.trans(0.0, 1.0).rot_deg(90.0),
            ctxt.rot_deg(90.0),
            ctxt.trans(1.0, 1.0).rot_deg(90.0),
            ctxt.trans(1.0, 0.0).rot_deg(90.0),
            ctxt.trans(0.0, 2.0),
            ctxt.trans(0.0, 1.0),
            ctxt,
        ];

        for i in 0..7 {
            self.draw_segment(DIGITS[digit as usize][i], ctxts[i], gl);
        }
    }

    fn draw_number(
        &self,
        mut number: usize,
        mut ctxt: graphics::context::Context,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        if number == 0 {
            self.draw_digit(0, ctxt, gl);
            return;
        }
        while number > 0 {
            self.draw_digit((number % 10) as u8, ctxt, gl);
            number = number / 10;
            use graphics::Transformed;
            ctxt = ctxt.trans(-1.5, 0.0);
        }
    }

    fn render(&self, mut ctxt: graphics::context::Context, gl: &mut opengl_graphics::GlGraphics) {
        use graphics::Transformed;
        let dims = ctxt.get_view_size();
        let scale = dims[0].min(dims[1]) / 290.0;
        ctxt = ctxt
            .scale(scale, scale)
            .trans(20.0, 277.5)
            .append_transform([[13.0, 5.0, 0.0], [0.0, -12.0, 0.0]]);

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GRAY: [f32; 4] = [0.7, 0.7, 0.7, 1.0];

        graphics::clear(BLACK, gl);

        graphics::Rectangle::new(GRAY).draw(
            [-0.5, -0.5, 11.0, 23.0],
            &ctxt.draw_state,
            ctxt.transform,
            gl,
        );
        graphics::Rectangle::new(BLACK).draw(
            [0.0, 0.0, 10.0, 22.0],
            &ctxt.draw_state,
            ctxt.transform,
            gl,
        );

        for x in 0..10 {
            for y in 0..22 {
                if let Some(tet) = self.board.board[x][y] {
                    let mut color = tetrhombino_color(tet);
                    color[3] = 0.5;
                    self.draw_rhomb((x as i8, y as i8), color, ctxt, gl);
                }
            }
        }

        if self.state == State::Falling || self.state == State::Loss {
            self.draw_tetrhombino(&self.board.current, ctxt, gl);
        }

        if self.state != State::Loss {
            self.draw_tetrhombino(
                &TetrhombinoState {
                    tetrhombino: self.next,
                    position: (-4, 16),
                    orientation: Orientation::Start,
                },
                ctxt,
                gl,
            );
        }

        self.draw_number(self.lines_cleared, ctxt.trans(15.0, 3.0), gl);
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
                gl.draw(r.viewport(), |c, gl| game.render(c, gl));
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
