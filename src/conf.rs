pub struct ScreenConf {
    pub size : (i16, i16),
    pub cell_size : (i16,i16),
    pub screen_size : (f32, f32),
    pub desired_fps : u32,
    pub tick_rate : f32
}
// dev conf
pub const GRID_SIZE: (i16, i16) = (10, 20);
pub const GRID_CELL_SIZE : (i16, i16) = (25, 25);
pub const SCREEN_SIZE : (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
    );
pub const DESIRED_FPS : u32 = 2;
pub const TICK_RATE : f32 = 0.5;



pub static SCREEN_CONF : ScreenConf = ScreenConf {
    size: GRID_SIZE,
    cell_size : GRID_CELL_SIZE,
    screen_size : SCREEN_SIZE,
    desired_fps : DESIRED_FPS,
    tick_rate : TICK_RATE
};

