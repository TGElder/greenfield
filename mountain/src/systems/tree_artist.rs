use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::trees::Drawing;
use crate::model::tree::Tree;

pub struct System {
    drawing: Option<Drawing>,
    update: bool,
    visible: bool,
}

impl System {
    pub fn new() -> System {
        System {
            drawing: None,
            update: true,
            visible: true,
        }
    }
    pub fn init(&mut self, graphics: &mut dyn Graphics) {
        self.drawing = Some(Drawing::init(graphics))
    }

    pub fn update(&mut self) {
        if self.visible {
            self.update = true;
        }
    }

    pub fn toggle_visible(&mut self, graphics: &mut dyn Graphics) {
        if self.visible {
            self.set_invisible(graphics);
        } else {
            self.set_visible();
        }
    }

    fn set_invisible(&mut self, graphics: &mut dyn Graphics) {
        self.visible = false;
        if let Some(drawing) = &mut self.drawing {
            drawing.hide(graphics);
        }
    }

    fn set_visible(&mut self) {
        self.visible = true;
        self.update = true;
    }

    pub fn run(
        &mut self,
        trees: &Grid<Option<Tree>>,
        terrain: &Grid<f32>,
        piste_map: &Grid<Option<usize>>,
        graphics: &mut dyn Graphics,
    ) {
        if self.visible && self.update {
            if let Some(drawing) = &mut self.drawing {
                drawing.draw(graphics, trees, terrain, piste_map);
            }
            self.update = false;
        }
    }
}
