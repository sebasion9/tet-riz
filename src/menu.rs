use ggez::graphics::{self, Color, TextFragment, PxScale};
use crate::conf::SCREEN_CONF;

pub struct TextBlock {
    pub block : graphics::Rect,
    pub text : graphics::Text,
    pub text_len : f32
}
impl TextBlock {
    pub fn new(x : f32, y: f32, w: f32, h: f32, contents : &str ) -> Self {
        let block = graphics::Rect::new(x, y, w, h);
        let text_scale = (2.0 * (w / contents.len() as f32)) - (w / 35.0);
        let text_len = text_scale * contents.len() as f32;
        let text = graphics::Text::new(TextFragment {
            text : contents.to_string(),
            color: Some(Color::WHITE),
            font : None,
            scale : Some(PxScale::from(text_scale))
        });
        TextBlock {
            block,
            text,
            text_len
        }
    }
}
pub struct Menu {
    pub modal_border : graphics::Rect,
    pub modal : graphics::Rect,
    pub text_blocks : Vec<TextBlock>
}
// hardcode based of screen width
impl Menu {
    pub fn new(text_blocks : Vec<TextBlock>) -> Self {
        let w = (SCREEN_CONF.screen_size.0 / 5.0) * 3.0;
        let h = (SCREEN_CONF.screen_size.1 / 10.0) * 8.0;
        let x = SCREEN_CONF.screen_size.0 / 5.0;
        let y = SCREEN_CONF.screen_size.1 / 10.0;
        Menu {
            modal_border : graphics::Rect { x, y, h, w },
            modal : graphics::Rect { x: x + 20.0, y: y + 20.0,  w: w- 40.0, h: h - 40.0  },
            text_blocks
        }
    }
    pub fn draw(&mut self, canvas : &mut graphics::Canvas) {
        let border_color = graphics::Color::from_rgb(23, 137, 112);
        let color = Color::from_rgb(23,137,27);
        canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                    .dest_rect(self.modal_border.into())
                    .color(border_color));
        canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                    .dest_rect(self.modal.into())
                    .color(color));
        for text_block in &self.text_blocks {
            canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                        .dest_rect(text_block.block)
                        .color(Color::from_rgb(23, 90, 137)));
            canvas.draw(&text_block.text, graphics::DrawParam::from([text_block.block.x, text_block.block.y]));
                        
        }

    }
}
