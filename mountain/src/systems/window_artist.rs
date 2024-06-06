use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::windows::Drawing;
use crate::model::building::Building;

pub struct System {
    drawing: Option<Drawing>,
    update: bool,
}

impl System {
    pub fn new() -> System {
        System {
            drawing: None,
            update: true,
        }
    }

    pub fn update(&mut self) {
        self.update = true;
    }

    pub fn init(&mut self, graphics: &mut dyn Graphics) {
        self.drawing = Some(Drawing::init(graphics));
    }

    pub fn run(&mut self, buildings: &HashMap<usize, Building>, graphics: &mut dyn Graphics) {
        if !self.update {
            return;
        }
        self.update = false;

        let Some(drawing) = self.drawing.as_ref() else {
            return;
        };
        drawing.draw(graphics, &mut buildings.values());
    }
}
