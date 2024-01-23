mod tetromino;
mod conf;
mod steering;
mod col;
mod block;
use crate::tetromino::Tetromino;
use crate::block::{Shape, Pos, DrawBlock, BlockIter};
use crate::conf::SCREEN_CONF;
use ggez::{
    event, graphics,
    Context, GameResult,
};
use steering::Direction;


struct GameState {
    tetromino : Tetromino,
    placed_blocks : Vec<Pos>,
    //gameover : bool
}
impl GameState {
    pub fn new() -> Self {
        GameState {
            tetromino : Tetromino::from_shape(&Shape::Long),
            placed_blocks : Vec::new(),
        }

    }
    pub fn push_tetr(&mut self, tetromino : Tetromino) {
        for block in tetromino.blocks {
            self.placed_blocks.push(block)
        }
    }
}
impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(SCREEN_CONF.desired_fps) {
            let current_tetr = &mut self.tetromino;
            // handling gameover (placed tetrs reached top)
            if current_tetr.is_colliding(&self.placed_blocks, &Direction::Down) && current_tetr.is_colliding_wall(0, &Direction::Up) {
                return Ok(())
            }
            // hanlding placing tetrs on the bottom
            if 
                current_tetr.is_colliding(&self.placed_blocks, &Direction::Down) || current_tetr.is_colliding_wall(SCREEN_CONF.size.1, &Direction::Down) {
                    // if collision, push to placed_blocks the current tetr
                    let placed = Tetromino::clone(&current_tetr);
                    self.push_tetr(placed);

                    // clearing line logic impl here
                    while let Some(y) = self.placed_blocks.line_clear(SCREEN_CONF.size.0, SCREEN_CONF.size.1) {
                        self.placed_blocks.cut_by_y(y);
                    }
                    // generating new tetr logic here
                    self.tetromino = Tetromino::from_shape(&Shape::random());
                }
            // if all went good updating current_tetr
            else {
                current_tetr.update();
            }
            
        }
        Ok(()) 
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        self.tetromino.draw(&mut canvas);
        if self.placed_blocks.len() > 0 {
            for block in &mut self.placed_blocks {
                block.draw(&mut canvas);
            }
        }

        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), ggez::GameError> {
        let current_tetr = &mut self.tetromino;
        let left_x = 0 as i16;
        let right_x = SCREEN_CONF.size.0;
        if let Some(dir) = input.keycode.and_then(Direction::from_keycode) {
            match dir {
                Direction::Left => {
                    if !current_tetr.is_colliding_wall(left_x, &dir) && !current_tetr.is_colliding(&self.placed_blocks, &dir) {
                        current_tetr.move_inline(&dir)
                    }
                },
                Direction::Right => {
                    if !current_tetr.is_colliding_wall(right_x, &dir) && !current_tetr.is_colliding(&self.placed_blocks, &dir){
                        current_tetr.move_inline(&dir)
                    }
                },
                Direction::Up => {
                    // adding 1 to look for collision "behind" the wall
                    current_tetr.turn();
                    while current_tetr.is_colliding_wall(left_x - 1, &Direction::Left) {                    
                        current_tetr.move_inline(&Direction::Right)
                    }
                    while current_tetr.is_colliding_wall(right_x + 1, &Direction::Right) {
                        current_tetr.move_inline(&Direction::Left)
                    }
                    while current_tetr.is_colliding_wall(SCREEN_CONF.size.1 + 1, &Direction::Down) {
                       current_tetr.lower(-1);
                    }
                }
                _=> todo!()
            }
        }
        Ok(()) 
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("snake", "awesome guy")
        .window_setup(ggez::conf::WindowSetup::default().title("tetris!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_CONF.screen_size.0, SCREEN_CONF.screen_size.1))
        .build()?;
    let state = GameState::new();
    event::run(ctx, event_loop, state)
}
