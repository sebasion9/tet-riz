use ggez::graphics;
use ggez::{
    input::keyboard::{KeyCode}
};
use oorandom::Rand32;
use getrandom;
use crate::conf::{SCREEN_CONF, ScreenConf};
use crate::steering::Direction;
pub struct Tetromino {
    st : GridPosition,
    nd : GridPosition,
    rd : GridPosition,
    th : GridPosition,
    shape : Shape
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
                   shape : Shape::Long
               }
            }
            &Shape::Z => {
               Tetromino {
                   st : GridPosition { x: 0, y: 0 },
                   nd: GridPosition { x: 0, y: 1 },
                   rd : GridPosition { x: 1, y: 1 },
                   th : GridPosition { x: 1, y: 2 },
                   shape : Shape::Z
               }
            }
            &Shape::T => {
                Tetromino {
                    st : GridPosition { x: 0, y: 0 },
                    nd: GridPosition { x: 1, y: 0 },
                    rd : GridPosition { x: 2, y: 0 },
                    th : GridPosition { x: 1, y: 1 },
                    shape : Shape::T
                }
            }
            &Shape::Square=> {
                Tetromino {
                    st : GridPosition { x: 0, y: 0 },
                    nd: GridPosition { x: 1, y: 0 },
                    rd : GridPosition { x: 0, y: 1 },
                    th : GridPosition { x: 1, y: 1 },
                    shape : Shape::Square
                }
            }
            &Shape::L => {
                Tetromino {
                    st : GridPosition { x: 0, y: 0 },
                    nd : GridPosition { x: 0, y: 1 },
                    rd : GridPosition { x: 0, y: 2 },
                    th : GridPosition { x: 1, y: 2 },
                    shape : Shape::L
                }
            }
        }
    }
    pub fn new(st : GridPosition, nd : GridPosition, rd : GridPosition, th : GridPosition, shape : Shape) -> Self {
        Tetromino {
            st,
            nd,
            rd,
            th,
            shape
        }
    }
    pub fn clone(obj : &Tetromino) -> Self {
        Tetromino::new(obj.st, obj.nd, obj.rd, obj.th, obj.shape)
    }
    pub fn get_tip(&self, dir : Direction) -> i16 {
        let mut extr = self.st;
        let x_s = [self.st.x, self.nd.x, self.rd.x, self.th.x];
        let y_s = [self.st.y, self.nd.y, self.rd.y, self.th.y];
        match dir {
            Direction::Up => {
                for y in y_s {
                    if y < extr.y {
                        extr.y = y; 
                    }
                }
                return extr.y
            },
            Direction::Down => {
                for y in y_s {
                    if y > extr.y {
                        extr.y = y;
                    }
                }
                return extr.y
            },
            Direction::Right => {
                for x in x_s {
                    if x > extr.x {
                        extr.x = x
                    }
                }
                return extr.x
            },
            Direction::Left => {
                for x in x_s {
                    if x < extr.x {
                        extr.x = x
                    }
                }
                return extr.x
            }
            _=> return extr.x
        }
    }
    pub fn is_colliding(&mut self, pos_y : i16) -> bool {
        let max = self.get_tip(Direction::Down);
        if max == pos_y - 1 {
            return true
        }
        else {
            return false
        }
    }

    
    pub fn draw(&mut self, canvas : &mut graphics::Canvas) {
        for i in 0..4 {
            let pos = match i {
                0 => self.st,
                1 => self.nd,
                2 => self.rd,
                3 => self.th,
                _=> self.st
            };
            let color = match self.shape {
                Shape::Long => graphics::Color::GREEN,
                Shape::Z => graphics::Color::RED,
                Shape::T => graphics::Color::BLUE,
                Shape::Square => graphics::Color::YELLOW,
                Shape::L => graphics::Color::MAGENTA
            };
            canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                        .dest_rect(pos.into())
                        .color(color));
        }
    }
    pub fn update(&mut self) {
        self.lower(1);
    }
    fn lower(&mut self, dist : i16)  {
        self.st.y = self.st.y + dist;
        self.nd.y = self.nd.y + dist;
        self.rd.y = self.rd.y + dist;
        self.th.y = self.th.y + dist;
    }

}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
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
            pos.x as i32 * SCREEN_CONF.cell_size.0 as i32,
            pos.y as i32 * SCREEN_CONF.cell_size.1 as i32,
            SCREEN_CONF.cell_size.0 as i32,
            SCREEN_CONF.cell_size.1 as i32,
        )
    }
}
#[derive(Clone, Copy)]
pub enum Shape {
    Long,
    T,
    Square,
    Z,
    L 
}
impl Shape {
    pub fn random() -> Shape{
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("couldnt generate RNG seed");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed)); 
        let random_num = rng.rand_range(0..5);
        println!("{}", random_num);
        match random_num {
            0 => Shape::Long,
            1 => Shape::T,
            2 => Shape::Square,
            3 => Shape::Z,
            4 => Shape::L,
            _ => Shape::Long
        }
    }
}








