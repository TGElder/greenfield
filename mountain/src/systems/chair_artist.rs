use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::chairs::Drawing;
use crate::model::frame::Frame;

pub struct System {
    drawing: Option<Drawing>,
}

impl System {
    pub fn new() -> System {
        System { drawing: None }
    }

    pub fn init(&mut self, graphics: &mut dyn Graphics) {
        self.drawing = Some(Drawing::init(graphics));
    }

    pub fn run(&mut self, frames: &HashMap<usize, Option<Frame>>, graphics: &mut dyn Graphics) {
        let Some(drawing) = self.drawing.as_ref() else {
            return;
        };
        drawing.draw(graphics, frames);
    }
}
