use ggez::graphics::{self, Color, TextFragment, PxScale};
use crate::conf::{SCREEN_CONF, color};

pub struct TextBlock {
    pub block : graphics::Rect,
    pub text : graphics::Text,
    pub text_scale : f32,
    pub text_len : f32,
    pub contents : String,
    pub is_selected : bool
}
impl TextBlock {
    pub fn new(x : f32, y: f32, w: f32, h: f32, contents : &str ) -> Self {
        let block = graphics::Rect::new(x, y, w, h);
        let text_scale = w / contents.len() as f32;
        let text_len = contents.len() as f32 * text_scale;
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
            text_len,
            contents : contents.to_string(),
            is_selected : false
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
    pub fn padding(&self) -> f32 {
        (self.block.w - self.text_len) / 2.0
    }
}
pub struct Menu {
    pub modal_border : graphics::Rect,
    pub modal : graphics::Rect,
    pub text_blocks : Vec<TextBlock>,
    pub selection_state : usize 
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
            text_blocks,
            selection_state : 1
        }
    }
    pub fn draw(&mut self, canvas : &mut graphics::Canvas) {
        let color = color::BLUE.to_rgb();
        let border_color = color::GRAY.to_rgb();
        let selected_color = color::PURPLE.to_rgb();
        canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                    .dest_rect(self.modal_border.into())
                    .color(border_color));
        canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                    .dest_rect(self.modal.into())
                    .color(color));
        for text_block in &self.text_blocks {
            if !text_block.is_selected {
                canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                            .dest_rect(text_block.block)
                            .color(border_color));
            }
            else {
                canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                            .dest_rect(text_block.block)
                            .color(selected_color));

            }
            canvas.draw(&text_block.text, graphics::DrawParam::from([text_block.block.x + text_block.padding(), text_block.block.y]));

        }

    }
    pub fn update_slow(&mut self, text_color : color::Color) {
        self.text_blocks[0].text_color(text_color);
    }
    pub fn update(&mut self) {
        for i in 1..self.text_blocks.len() {
            self.text_blocks[i].text_color(color::WHITE);
            self.text_blocks[self.selection_state].text_color(color::YELLOW);
        }
    }
}
