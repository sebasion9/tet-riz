mod tetromino;
mod conf;
mod steering;
mod col;
mod block;
mod menu;

use crate::tetromino::Tetromino;
use crate::block::{Shape, Pos, DrawBlock, BlockIter};
use crate::conf::{SCREEN_CONF, color};
use ggez::event::MouseButton;
use ggez::{
    event, graphics,
    Context, GameResult,
};
use menu::TextBlock;
use steering::Direction;
#[derive(PartialEq)]
enum Mode{
    MenuLoop,
    GameLoop
}
struct GameState {
    tetromino : Tetromino,
    placed_blocks : Vec<Pos>,
    mode : Mode,
    menu : menu::Menu,
    blink_flag : bool
}
impl GameState {
    pub fn new() -> Self {
        let tetromino = Tetromino::from_shape(&Shape::Long);
        let w = (SCREEN_CONF.screen_size.0 / 5.0) * 3.0;
        let _h = (SCREEN_CONF.screen_size.1 / 10.0) * 8.0;
        let x = SCREEN_CONF.screen_size.0 / 5.0;
        let y = SCREEN_CONF.screen_size.1 / 10.0;
        GameState {
            tetromino,
            placed_blocks : Vec::new(),
            mode : Mode::MenuLoop,
            blink_flag : false,
            menu : menu::Menu::new(vec![
                                   TextBlock::new(x + 30.0, y + 50.0, w - 60.0, 40.0, "TETRUST"),
                                   TextBlock::new(x + 30.0, y + 120.0, w - 60.0, 40.0, "NEW GAME"),
                                   TextBlock::new(x + 30.0, y + 190.0, w - 60.0, 40.0, "OPTIONS")
            ])
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
        match self.mode {
            Mode::MenuLoop => {
                if ctx.time.check_update_time(SCREEN_CONF.desired_fps) {
                    self.blink_flag = !self.blink_flag; 
                    if self.blink_flag {
                        self.menu.update_slow(color::GREEN);
                    }
                    else {
                        self.menu.update_slow(color::WHITE);
                    }
                }
                if ctx.time.check_update_time(SCREEN_CONF.desired_fps * 100) {
                    self.menu.update();
                    self.menu.text_blocks.iter_mut().for_each(|text_block| text_block.is_selected = false);
                    self.menu.text_blocks[self.menu.selection_state].is_selected = true;
                }
                Ok(())
            },
            Mode::GameLoop => {
                if ctx.time.check_update_time(SCREEN_CONF.desired_fps) {
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
                            let random_shape = Shape::random();
                            self.tetromino = Tetromino::from_shape(&random_shape);
                            self.tetromino.shadow_blocks = Some(self.tetromino.create_shadow(&self.placed_blocks, SCREEN_CONF.size.1));
                        }
                    // if all went good updating current_tetr
                    else {
                        current_tetr.update();
                    }
                    return Ok(())
                }
                if ctx.time.check_update_time(SCREEN_CONF.desired_fps * 100) {
                    self.tetromino.shadow_blocks = Some(self.tetromino.create_shadow(&self.placed_blocks, SCREEN_CONF.size.1));
                }
                Ok(())

            }
        }
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, color::BLACK);
        match self.mode {
            Mode::GameLoop => {
                self.tetromino.draw(&mut canvas);
                if self.placed_blocks.len() > 0 {
                    for block in &mut self.placed_blocks {
                        block.draw(&mut canvas);
                    }
                }
            },
            Mode::MenuLoop => {
                self.menu.draw(&mut canvas);
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
        if self.mode == Mode::MenuLoop {
            if let Some(dir) = input.keycode.and_then(Direction::from_keycode) {
                match dir {
                    Direction::Down => {
                        if self.menu.selection_state < self.menu.text_blocks.len() - 1 {
                            self.menu.selection_state += 1;    
                        }
                    },
                    Direction::Up => {
                        if self.menu.selection_state > 1 {
                            self.menu.selection_state -= 1;
                        }
                    }
                    Direction::Space => {
                        if self.menu.selection_state == 1 {
                            self.mode = Mode::GameLoop;
                        }
                    }
                    _=> {

                    }

                }
            }
            return Ok(())
        }
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
                },
                Direction::Down => {
                    if !(current_tetr.is_colliding_wall(SCREEN_CONF.size.1, &dir) || current_tetr.is_colliding(&self.placed_blocks, &dir)){
                        current_tetr.lower(1);
                    }
                },
                Direction::Space => {
                    while !(current_tetr.is_colliding_wall(SCREEN_CONF.size.1, &Direction::Down) || current_tetr.is_colliding(&self.placed_blocks, &Direction::Down)){
                        current_tetr.lower(1);
                    }
                    let placed = Tetromino::clone(&current_tetr);
                    self.push_tetr(placed);

                    // clearing line logic impl here
                    while let Some(y) = self.placed_blocks.line_clear(SCREEN_CONF.size.0, SCREEN_CONF.size.1) {
                        self.placed_blocks.cut_by_y(y);
                    }
                    // generating new tetr logic here
                    self.tetromino = Tetromino::from_shape(&Shape::random());
                    self.tetromino.shadow_blocks = Some(self.tetromino.create_shadow(&self.placed_blocks, SCREEN_CONF.size.1));


                },
            }
        }
        Ok(()) 
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
        ) -> Result<(), ggez::GameError> {
        let play_button = self.menu.text_blocks[1].block;
        if !(button == MouseButton::Left && self.mode == Mode::MenuLoop) {
            return Ok(()) 
        }
        if x > play_button.x && x < (play_button.x + play_button.w) && y > play_button.y && y < (play_button.y + play_button.h) {
            self.mode = Mode::GameLoop;
        }
        Ok(())
    }

}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("tetris", "awesome guy")
        .window_setup(ggez::conf::WindowSetup::default().title("tetris!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_CONF.screen_size.0, SCREEN_CONF.screen_size.1))
        .build()?;
    let state = GameState::new();
    ctx.gfx.add_font("retro_pixel", graphics::FontData::from_slice(include_bytes!("pub\\jostix.otf")).unwrap());
    event::run(ctx, event_loop, state)
}





