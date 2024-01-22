use ggez::graphics;
use oorandom::Rand32;
use getrandom;
use crate::conf::SCREEN_CONF;
use crate::steering::Direction;
use crate::col::are_colliding;
#[derive(Debug)]
pub struct Tetromino {
    pub blocks: [Pos; 4],
    shape : Shape
}
impl Tetromino {
    // constructors
    pub fn from_shape(shape : &Shape) -> Self {
        match shape{
            &Shape::Long => {
               Tetromino {
                    blocks : [ 
                    Pos { x: 0, y: 0 },
                    Pos { x: 1, y: 0},
                    Pos { x : 2, y : 0},
                    Pos { x :3, y: 0}],
                    shape : Shape::Long
               }
            }
            &Shape::Z => {
               Tetromino {
                   blocks : [
                       Pos { x: 0, y: 0 },
                       Pos { x: 0, y: 1 },
                       Pos { x: 1, y: 1 },
                       Pos { x: 1, y: 2 }
                   ],
                   shape : Shape::Z
               }
            }
            &Shape::T => {
                Tetromino {
                    blocks  :[
                        Pos { x: 0, y: 0 },
                        Pos { x: 1, y: 0 },
                        Pos { x: 2, y: 0 },
                        Pos { x: 1, y: 1 },
                    ],
                    shape : Shape::T
                }
            }
            &Shape::Square=> {
                Tetromino {
                    blocks : [
                        Pos { x: 0, y: 0 },
                        Pos { x: 1, y: 0 },
                        Pos { x: 0, y: 1 },
                        Pos { x: 1, y: 1 },
                    ],
                    shape : Shape::Square
                }
            }
            &Shape::L => {
                Tetromino {
                    blocks : [
                        Pos { x: 0, y: 0 },
                        Pos { x: 0, y: 1 },
                        Pos { x: 0, y: 2 },
                        Pos { x: 1, y: 2 },
                    ],
                    shape : Shape::L
                }
            }
        }
    }
    pub fn new(st : Pos, nd : Pos, rd : Pos, th : Pos, shape : Shape) -> Self {
        Tetromino {
            blocks : [
                st,
                nd,
                rd,
                th,
            ],
            shape
        }
    }
    pub fn clone(obj : &Tetromino) -> Self {
        Tetromino::new(obj.blocks[0], obj.blocks[1], obj.blocks[2], obj.blocks[3], obj.shape)
    }

    // collision logic 
    pub fn is_colliding (&self, placed_blocks : &Vec<Pos>, dir : &Direction) -> bool {
        are_colliding(&self.blocks.to_vec(), placed_blocks, dir)
    }
    pub fn is_colliding_wall(&self, wall_cord: i16, dir : &Direction) -> bool {
        match dir {
            Direction::Down => {
                for block in self.blocks {
                    if block.y == wall_cord - 1 {
                        return true
                    }
                }
            },
            Direction::Up => {
                for block in self.blocks {
                    if block.y == wall_cord {
                        return true
                    }
                }
            },
            Direction::Left => {
                for block in self.blocks {
                    if block.x == wall_cord {
                        return true
                    }
                }
            },
            Direction::Right => {
                for block in self.blocks {
                    if block.x == wall_cord - 1 {
                        return true
                    }
                }
            },
            _=> return false 
        }
        return false
    }
    // 
    pub fn draw(&mut self, canvas : &mut graphics::Canvas) {
        for i in 0..4 {
            let color = match self.shape {
                Shape::Long => graphics::Color::GREEN,
                Shape::Z => graphics::Color::RED,
                Shape::T => graphics::Color::BLUE,
                Shape::Square => graphics::Color::YELLOW,
                Shape::L => graphics::Color::MAGENTA
            };
            canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                        .dest_rect(self.blocks[i].into())
                        .color(color));
        }
    }
    pub fn update(&mut self) {
        self.lower(1);
    }
    // movement
    pub fn move_inline(&mut self, dir : &Direction) {
        match dir {
            Direction::Left => {
                for block in self.blocks.iter_mut(){
                    block.x -= 1;
                }
            },
            Direction::Right => {
                for block in self.blocks.iter_mut() {
                    block.x += 1;
                }
            },
            _ => todo!()
        }
    }
    fn lower(&mut self, dist : i16)  {
        for block in self.blocks.iter_mut() {
            block.y += dist;
        }
    }

}
//
//
#[derive(PartialEq)]
pub struct LineClearResult {
    y_cord : i16,
    tetr_indexes : Vec<i16>
}
impl LineClearResult {
    pub fn new(y_cord : i16, tetr_indexes : Vec<i16>) -> Self {
        LineClearResult {
            y_cord,
            tetr_indexes
        }
    }
}
pub trait TetrIter {
    //#1 find if any line and which line should be cleared
    //#2 find what tetrs should be cleared
    //#3 call another function to clear the tetrs, and lower the tetrs on top
    fn line_clear(&self, board_width : i16, board_height : i16) -> Option<LineClearResult>;
    fn max_y(&self, max_height : i16) -> i16;
}
impl TetrIter for Vec<Pos> {
    fn max_y(&self, max_height : i16) -> i16 {
        let mut max = max_height;
        if self.len() < 1 {
            return max
        }
        for block in self {
            if block.y < max {
                max = block.y;
            }
        }
        max
    }
    fn line_clear(&self, bw: i16, bh : i16) -> Option<LineClearResult> {
        if self.len() < 1 {
            println!("no line clear");
            return None
        }
        for i in self.max_y(bh)..bh {
            let mut block_count : i16 = 0;
            let mut indexes : Vec<i16> = Vec::new();
            for block in 0..self.len()-1 {
                if self[block].y == i {
                    block_count += 1;
                    indexes.push(block.try_into().unwrap());
                }
                if block_count == bw {
                    println!("line clear");
                    return Some(LineClearResult::new(i, indexes))
                }
            }
            indexes.clear();
        }
        println!("no line clear");
        return None
    }
}
//
//
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pos {
    pub x:i16,
    pub y:i16
}
pub trait DrawBlock {
    fn draw(&self, canvas : &mut graphics::Canvas);
}
impl DrawBlock for Pos {
    fn draw(&self, canvas : &mut graphics::Canvas) {
        canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                    .dest_rect((*self).into())
                    .color(graphics::Color::CYAN));
    }
}
impl Pos {
    pub fn new(x:i16, y:i16) -> Self{
        Pos { x, y }
    }
}
impl From<(i16, i16)> for Pos {
    fn from(pos: (i16, i16)) -> Self {
        Pos { x: pos.0, y: pos.1 }
    }
}
impl From<Pos> for graphics::Rect {
    fn from(pos: Pos) -> Self {
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn line_clear_filled_fully() {
        let width : i16 = 2;
        let height : i16 = 5;
        let placed_blocks : Vec<Pos> = vec![
            Pos::new(0, 0),
            Pos::new(1, 0),
            Pos::new(2, 0)
        ];
        let result = placed_blocks.line_clear(width, height);
        assert!(result == Some(LineClearResult::new(0,vec![0,1])));
    }
    #[test]
    fn line_clear_filled_partially() {
        let width : i16 = 2;
        let height : i16 = 5;
        let placed_blocks : Vec<Pos> = vec![
            Pos::new(0,0),
            Pos::new(0,1),
            Pos::new(1,0)
        ];
        let result = placed_blocks.line_clear(width, height);
        assert!(result == None)


    }
    #[test]
    fn line_clear_no_fill() {
        let width : i16 = 2;
        let height : i16 = 5;
        let placed_blocks : Vec<Pos> = Vec::new();
        let result = placed_blocks.line_clear(width, height);
        assert!(result == None)
    }
    #[test]
    fn line_clear_dif_y() {
        let width : i16 = 2;
        let height : i16 = 5;
        let placed_blocks : Vec<Pos> = vec![
            Pos::new(0,0),
            Pos::new(1,1),
            Pos::new(2,1)
        ];
        let result = placed_blocks.line_clear(width, height);
        assert!(result == None)
    }
}






