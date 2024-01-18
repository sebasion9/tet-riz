use ggez::graphics;
use ggez::{
    input::keyboard::{KeyCode}
};
use oorandom::Rand32;
use getrandom;
use crate::conf::{SCREEN_CONF, ScreenConf};
use crate::steering::Direction;
#[derive(Debug)]
pub struct Tetromino {
    st : GridPosition,
    nd : GridPosition,
    rd : GridPosition,
    th : GridPosition,
    shape : Shape
}
impl Tetromino {
    // init self methods 
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
    // collision logic 
    pub fn is_colliding(&self, under: Vec<Tetromino>, xs : Vec<i16>,  board_size : (i16,i16)) -> bool {
        //println!("{}", under.len());
        let mut highest : Vec<GridPosition> = Vec::new(); 
        let falling_cells = self.get_extr_cells(true);
        for tetr in under {
            let high_cells = tetr.get_extr_cells(false);
            for cell in high_cells {
                for x in &xs {
                    if cell.x == *x {
                        highest.push(cell);
                    }
                }
            }
        }
        let mut is_colliding : bool = false;
        if falling_cells.len() <= highest.len() {
            for cell in falling_cells {
                for bot_cell in &highest {
                    if cell.x == bot_cell.x && cell.y == bot_cell.y - 1 {
                        is_colliding = true;
                        break;
                    }
                }
            }
        }
        else {
            for cell in falling_cells {
                if cell.y == board_size.1 - 1 {
                    is_colliding = true;
                    break;
                }
            }
        }
        is_colliding
    }
    pub fn get_extr_cells(&self, is_falling : bool) -> Vec<GridPosition> {
        let mut max = 0;
        let mut min = SCREEN_CONF.size.1;
        let mut res : Vec<GridPosition> = Vec::new();
        let cell_pos = [self.st, self.nd, self.rd, self.th];
        if is_falling {
            for pos in cell_pos {
                if pos.y > max {
                    max = pos.y;
                }
            }
            for pos in cell_pos {
                if pos.y == max {
                    res.push(pos);
                }
            }

        }
        else {
            for pos in cell_pos {
                if pos.y < min {
                    min = pos.y;
                }
            }
            for pos in cell_pos {
                if pos.y == min{
                    res.push(pos);
                }
            }
        }
        res
    }
    pub fn get_tetr_under(&self, falling_cells : &Vec<GridPosition>, placed_tetr : &Vec<Tetromino>) -> (Vec<Tetromino>, Vec<i16>) {
        let mut tetr_under : Vec<Tetromino> = Vec::new();
        let mut xs : Vec<i16> = Vec::new();
        for pos in falling_cells {
            xs.push(pos.x);
        }
        if placed_tetr.len() > 0 {
            for tetr_index in 0..placed_tetr.len()-1 {
                let tetr = &placed_tetr[tetr_index];
                for x in &xs {
                    if tetr.st.x == *x || tetr.nd.x == *x || tetr.rd.x == *x || tetr.th.x == *x {
                        tetr_under.push(Tetromino::clone(&tetr));
                    }
                }
            }

        }
        (tetr_under, xs)

    }
    
    // render methods 
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
    pub x:i16,
    pub y:i16
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
#[derive(Clone, Copy, Debug)]
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








