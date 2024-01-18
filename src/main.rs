mod tetromino;
mod conf;
mod steering;
//use oorandom::Rand32;
use crate::tetromino::{Tetromino, GridPosition, Shape};
use crate::conf::SCREEN_CONF;
use ggez::{
    event, graphics,
    //input::keyboard::{KeyCode, KeyInput},
    Context, GameResult, GameError
};


struct GameState {
    tetromino : Tetromino,
    placed_tetr : Vec<Tetromino>
}
impl GameState {
    pub fn new() -> Self {
        GameState {
            tetromino : Tetromino::from_shape(&Shape::Long),
            placed_tetr : Vec::new()
        }

    }
    pub fn push_tetr(&mut self, tetromino : Tetromino) {
        self.placed_tetr.push(tetromino);
    }
}
impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(SCREEN_CONF.desired_fps) {
            let current_tetr = &mut self.tetromino;
            let extr_cells = current_tetr.get_extr_cells(true);
            if extr_cells.len() > 0  {
                let (under,xs) = current_tetr.get_tetr_under(&extr_cells, &self.placed_tetr);
                if  current_tetr.is_colliding(under, xs, SCREEN_CONF.size) {
                    let placed = Tetromino::clone(&current_tetr);
                    self.push_tetr(placed);
                    self.tetromino = Tetromino::from_shape(&Shape::random());
                }
                else {
                    current_tetr.update();
                }
            }
            
        }
        Ok(()) 
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        self.tetromino.draw(&mut canvas);
        if self.placed_tetr.len() > 0 {
            for tetr in &mut self.placed_tetr {
                tetr.draw(&mut canvas);
            }
        }

        canvas.finish(ctx)?;
        ggez::timer::yield_now();
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
