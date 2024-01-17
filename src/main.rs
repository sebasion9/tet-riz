use oorandom::Rand32;
use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult, GameError
};

const GRID_SIZE: (i16, i16) = (10, 20);
const GRID_CELL_SIZE : (i16, i16) = (25, 25);
const SCREEN_SIZE : (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);
const DESIRED_FPS : u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x:i16,
    y:i16
}
impl GridPosition {
    pub fn new(x:i16, y:i16) -> Self{
        GridPosition { x, y }
    }
}
impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}
impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}
//#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Space
}
impl Direction {
    pub fn from_keycode(key : KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            KeyCode::Space => Some(Direction::Space),
            _=> None
        }
    }
}
struct Tetromino {
    st : GridPosition,
    nd : GridPosition,
    rd : GridPosition,
    th : GridPosition
}
impl Tetromino {
    pub fn from_shape(shape : &Shape) -> Self {
        match shape{
            &Shape::Long => {
               Tetromino {
                   st : GridPosition { x: 0, y: 0 },
                   nd: GridPosition { x: 1, y: 0 },
                   rd : GridPosition { x: 2, y: 0 },
                   th : GridPosition { x: 3, y: 0 },
               }
            }
            &Shape::Z => {
               Tetromino {
                   st : GridPosition { x: 0, y: 0 },
                   nd: GridPosition { x: 0, y: 1 },
                   rd : GridPosition { x: 1, y: 1 },
                   th : GridPosition { x: 1, y: 2 },
               }
            }
            &Shape::T => {
                Tetromino {
                    st : GridPosition { x: 0, y: 0 },
                    nd: GridPosition { x: 1, y: 0 },
                    rd : GridPosition { x: 2, y: 0 },
                    th : GridPosition { x: 1, y: 1 },
                }
            }
            &Shape::Square=> {
                Tetromino {
                    st : GridPosition { x: 0, y: 0 },
                    nd: GridPosition { x: 1, y: 0 },
                    rd : GridPosition { x: 0, y: 1 },
                    th : GridPosition { x: 1, y: 1 },
                }
            }
            &Shape::L => {
                Tetromino {
                    st : GridPosition { x: 0, y: 0 },
                    nd : GridPosition { x: 0, y: 1 },
                    rd : GridPosition { x: 0, y: 2 },
                    th : GridPosition { x: 1, y: 2 },
                }
            }
        }
    }
    pub fn new(st : GridPosition, nd : GridPosition, rd : GridPosition, th : GridPosition) -> Self {
        Tetromino {
            st,
            nd,
            rd,
            th
        }
    }
    fn draw(&self, canvas : &mut graphics::Canvas) {
        for i in 0..4 {
            let pos = match i {
                0 => self.st,
                1 => self.nd,
                2 => self.rd,
                3 => self.th,
                _=> self.st
            };
            canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                        .dest_rect(pos.into())
                        .color([0.3,0.3,0.0,1.0]));
        }
    }
}
enum Shape {
    Long,
    T,
    Square,
    Z,
    L 
}
struct GameState {
    tetromino : Tetromino
}
impl GameState {
    pub fn new() -> Self {
        GameState {
            tetromino : Tetromino::from_shape(&Shape::Square)
        }

    }
}
impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(()) 
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        self.tetromino.draw(&mut canvas);
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("snake", "awesome guy")
        .window_setup(ggez::conf::WindowSetup::default().title("tetris!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;
    let state = GameState::new();
    event::run(ctx, event_loop, state)
}
