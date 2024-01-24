use ggez::graphics;
use crate::block::{Shape,Pos};
use crate::steering::Direction;
use crate::col::are_colliding;
#[derive(Debug)]
pub struct Tetromino {
    pub blocks: [Pos; 4],
    pub shadow_blocks : Option<[Pos; 4]>,
    shape : Shape,
    turn_state : u32
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
                        shape : Shape::Long,
                        turn_state : 0,
                        shadow_blocks : None
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
                    shape : Shape::Z,
                    turn_state : 0,
                    shadow_blocks : None
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
                    shape : Shape::T,
                    turn_state : 0,
                    shadow_blocks : None

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
                    shape : Shape::Square,
                    turn_state : 0,
                    shadow_blocks : None
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
                    shape : Shape::L,
                    turn_state : 0,
                    shadow_blocks : None
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
            shape,
            turn_state : 0,
            shadow_blocks : None
        }
    }
    pub fn clone(obj : &Tetromino) -> Self {
        Tetromino::new(obj.blocks[0], obj.blocks[1], obj.blocks[2], obj.blocks[3], obj.shape)
    }
    pub fn create_shadow(&self, placed_blocks : &Vec<Pos>, wall_cord: i16) -> [Pos;4] {
        let mut shadow = Tetromino::clone(self);
        while !(shadow.is_colliding(placed_blocks, &Direction::Down) || shadow.is_colliding_wall(wall_cord, &Direction::Down)) {
            shadow.lower(1);
        }
        shadow.blocks
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
            if let Some(shadow) = self.shadow_blocks {
                canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                            .dest_rect(shadow[i].into())
                            .color(graphics::Color::WHITE));
            }
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
    pub fn lower(&mut self, dist : i16)  {
        for block in self.blocks.iter_mut() {
            block.y += dist;
        }
    }
    // hardcoding the turning logic because no idea what algorithm would it even be
    pub fn turn(&mut self) {
        match self.shape {
            Shape::Long => {
                match self.turn_state {
                    0  => {
                        self.turn_state = 1;

                        self.blocks[0].y += -1;
                        self.blocks[0].x += 1;

                        self.blocks[2].y += 1;
                        self.blocks[2].x += -1;

                        self.blocks[3].y += 2;
                        self.blocks[3].x += -2;
                    },
                    1 => {
                        self.turn_state = 0;

                        self.blocks[0].y += 1;
                        self.blocks[0].x += -1;

                        self.blocks[2].y += -1;
                        self.blocks[2].x += 1;
                        
                        self.blocks[3].y += -2;
                        self.blocks[3].x += 2;
                    }
                    _ => {
                        self.turn_state = 0;
                    }
                }
            },
            Shape::Square => {
                if self.turn_state == 0 {
                    self.turn_state = 1;
                }
                self.turn_state = 0;
            },
            Shape::T => {
                match self.turn_state {
                    0 => {
                        self.turn_state = 1; 

                        self.blocks[0].y += -1;
                        self.blocks[0].x += 1;

                        self.blocks[2].y += 1;
                        self.blocks[2].x += -1;

                        self.blocks[3].y += -1;
                        self.blocks[3].x += -1;
                    },
                    1 => {
                        self.turn_state = 2;

                        self.blocks[0].y += 1;
                        self.blocks[0].x += 1;

                        self.blocks[2].y += -1;
                        self.blocks[2].x += -1;

                        self.blocks[3].y += -1;
                        self.blocks[3].x += 1;
                        

                    },
                    2 => {
                        self.turn_state = 3;

                        self.blocks[0].y += 1;
                        self.blocks[0].x += -1;

                        self.blocks[2].y += -1;
                        self.blocks[2].x += 1;
                        
                        self.blocks[3].y += 1;
                        self.blocks[3].x += 1;

                    },
                    3 => {
                        self.turn_state = 0;

                        self.blocks[0].y += -1;
                        self.blocks[0].x += -1;

                        self.blocks[2].y += 1;
                        self.blocks[2].x += 1;

                        self.blocks[3].y += 1;
                        self.blocks[3].x += -1;


                    },
                    _ => {
                        self.turn_state = 0;
                    }
                }
            },
            Shape::Z => {
                match self.turn_state {
                    0 => {
                        self.turn_state = 1;

                        self.blocks[0].x += 2;

                        self.blocks[1].y += -1;
                        self.blocks[1].x += 1;

                        self.blocks[3].y += -1;
                        self.blocks[3].x += -1;
                    },
                    1 => {
                        self.turn_state = 0;

                        self.blocks[0].x += -2;
                        
                        self.blocks[1].y += 1;
                        self.blocks[1].x += -1;

                        self.blocks[3].y += 1;
                        self.blocks[3].x += 1;

                    },
                    _=> {
                        self.turn_state = 0;
                    }

                }
            },
            Shape::L => {
                match self.turn_state {
                    0 => {
                        self.turn_state = 1;

                        self.blocks[0].y += 1;
                        self.blocks[0].x += 2;

                        self.blocks[1].x += 1;

                        self.blocks[2].y += -1;
                        
                        self.blocks[3].x += -1;
                    },
                    1 => {
                        self.turn_state = 2;

                        self.blocks[0].y += 1;

                        self.blocks[1].x += 1;

                        self.blocks[2].y += -1;
                        self.blocks[2].x += 2;

                        self.blocks[3].y += -2;
                        self.blocks[3].x += 1;
                    },
                    2 => {
                        self.turn_state = 3;

                        self.blocks[0].x += -2;

                        self.blocks[1].y += 1;
                        self.blocks[1].x += -1;

                        self.blocks[2].y += 2;

                        self.blocks[3].y += 1;
                        self.blocks[3].x += 1;

                    },
                    3 => {
                        self.turn_state = 0;

                        self.blocks[0].y += -2;

                        self.blocks[1].y += -1;
                        self.blocks[1].x += -1;

                        self.blocks[2].x += -2;

                        self.blocks[3].y += 1;
                        self.blocks[3].x += -1;
                    },
                    _ => self.turn_state = 0


                }

            }
        }
    }

}
//
//

