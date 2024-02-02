use ggez::graphics;
use oorandom::Rand32;
use crate::conf::{SCREEN_CONF, color};
pub trait BlockIter {
    fn line_clear(&self, board_width : i16, board_height : i16) -> Option<i16>;
    fn max_y(&self, max_height : i16) -> i16;
    fn cut_by_y(&mut self, cord : i16) -> &mut Self;
}
impl BlockIter for Vec<Pos> {
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
    fn line_clear(&self, bw: i16, bh : i16) -> Option<i16> {
        if self.len() < 1 {
            return None
        }
        for i in self.max_y(bh)..bh {
            let mut block_count : i16 = 0;
            for block in 0..self.len() {
                if self[block].y == i {
                    block_count += 1;
                }
                if block_count == bw {
                    return Some(i)
                }
            }
        }
        return None
    }
    fn cut_by_y(&mut self, cord : i16) ->  &mut Self {
        let mut i = self.len();
        while i > 0 {
            i -= 1;
            if self[i].y == cord {
                self.remove(i);
            }
            else if self[i].y < cord {
                self[i].y += 1;
            }
        }
        self
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pos {
    pub x:i16,
    pub y:i16,
    pub color : color::Color
}
pub trait DrawBlock {
    fn draw(&self, canvas : &mut graphics::Canvas);
}
impl DrawBlock for Pos {
    fn draw(&self, canvas : &mut graphics::Canvas) {
        canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                    .dest_rect((*self).into())
                    .color(self.color.to_rgb()));
    }
}
impl Pos {
    pub fn new(x:i16, y:i16) -> Self{
        Pos { x, y, color : color::WHITE }
    }
}
impl From<(i16, i16)> for Pos {
    fn from(pos: (i16, i16)) -> Self {
        Pos { x: pos.0, y: pos.1, color : color::WHITE}
    }
}
impl From<Pos> for graphics::Rect {
    fn from(pos: Pos) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * SCREEN_CONF.cell_size.0 as i32 + 1,
            pos.y as i32 * SCREEN_CONF.cell_size.1 as i32 + 1,
            SCREEN_CONF.cell_size.0 as i32 - 2,
            SCREEN_CONF.cell_size.1 as i32 - 2,
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








