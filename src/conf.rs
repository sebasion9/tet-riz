use ggez::graphics;

pub struct ScreenConf {
    pub size : (i16, i16),
    pub cell_size : (i16,i16),
    pub screen_size : (f32, f32),
    pub desired_fps : u32,
    pub tick_rate : f32
}
// dev conf
pub const GRID_SIZE: (i16, i16) = (10, 20);
pub const GRID_CELL_SIZE : (i16, i16) = (40, 40);
pub const SCREEN_SIZE : (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
    );
pub const DESIRED_FPS : u32 = 2;
pub const TICK_RATE : f32 = 0.5;



pub const SCREEN_CONF : ScreenConf = ScreenConf {
    size: GRID_SIZE,
    cell_size : GRID_CELL_SIZE,
    screen_size : SCREEN_SIZE,
    desired_fps : DESIRED_FPS,
    tick_rate : TICK_RATE
};

pub mod color{
    use super::*;
    type RGB = (u8, u8, u8);
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Color {
        WHITE(RGB),
        BLACK(RGB),
        GREEN(RGB),
        BLUE(RGB),
        RED(RGB),
        YELLOW(RGB),
        PURPLE(RGB),
        GRAY(RGB)
    }
    impl Color {
        pub fn to_rgb(self) -> (u8, u8, u8) {
            match self {
                Color::WHITE(rgb) | 
                    Color::PURPLE(rgb) |
                    Color::GRAY(rgb) |
                    Color::BLACK(rgb) |
                    Color::GREEN(rgb) |
                    Color::BLUE(rgb) |
                    Color::RED(rgb) |
                    Color::YELLOW(rgb) => (rgb.0, rgb.1, rgb.2)
            }
                    
        }
    }
    impl Into<Option<graphics::Color>> for Color {
        fn into(self) -> Option<graphics::Color> {
            match self {
                Color::WHITE(rgb) | 
                    Color::PURPLE(rgb) |
                    Color::BLACK(rgb) |
                    Color::GREEN(rgb) |
                    Color::BLUE(rgb) |
                    Color::RED(rgb) |
                    Color::GRAY(rgb) |
                    Color::YELLOW(rgb) => Some(graphics::Color::new(rgb.0 as f32, rgb.1 as f32, rgb.2 as f32, 0.0))
            } 
        }
    }
    pub const WHITE : Color = Color::WHITE((255,255,255));
    pub const BLACK : Color = Color::BLACK((0,0,0));

    pub const GREEN : Color = Color::GREEN((0,255,0));
    pub const BLUE : Color = Color::BLUE((32, 32, 255));
    pub const RED : Color = Color::RED((255,52,7));
    pub const YELLOW : Color = Color::YELLOW((255,255,0));
    pub const PURPLE : Color = Color::PURPLE((210,0,255));
    pub const GRAY : Color = Color::GRAY((90,90,90));


}
