use ggez::graphics::{self, Color, TextFragment, PxScale};
use crate::conf::{SCREEN_CONF, color};

pub struct TextBlock {
    pub block : graphics::Rect,
    pub text : graphics::Text,
    pub text_scale : f32,
    pub contents : String
}
impl TextBlock {
    pub fn new(x : f32, y: f32, w: f32, h: f32, contents : &str ) -> Self {
        let block = graphics::Rect::new(x, y, w, h);
        let text_scale = ((w / contents.len() as f32) * 3.0)/ 2.0;
        let text = graphics::Text::new(TextFragment {
            text : contents.to_string(),
            color: Some(Color::WHITE),
            font : Some("retro_pixel".to_string()),
            scale : Some(PxScale::from(text_scale))
        });
        
        TextBlock {
            block,
            text,
            text_scale,
            contents : contents.to_string()
        }
    }
    pub fn text_color(&mut self, text_color : color::Color)  {
        let text = graphics::Text::new(TextFragment {
            text : self.contents.clone(),
            color : Some(text_color.to_rgb().into()),
            font : Some("retro_pixel".to_string()),
            scale : Some(PxScale::from(self.text_scale))

        });
        self.text = text;
    }
}
pub struct Menu {
    pub modal_border : graphics::Rect,
    pub modal : graphics::Rect,
    pub text_blocks : Vec<TextBlock>
}
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
        let color = color::BLUE.to_rgb();
        let border_color = color::GRAY.to_rgb();
        canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                    .dest_rect(self.modal_border.into())
                    .color(border_color));
        canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                    .dest_rect(self.modal.into())
                    .color(color));
        for text_block in &self.text_blocks {
            canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                        .dest_rect(text_block.block)
                        .color(border_color));
            canvas.draw(&text_block.text, graphics::DrawParam::from([text_block.block.x, text_block.block.y]));
                        
        }

    }
    pub fn update(&mut self, text_color : color::Color) {
        for block in self.text_blocks.iter_mut() {
            block.text_color(text_color);
        }
    }
}
