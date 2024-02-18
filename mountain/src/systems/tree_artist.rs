use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::trees::Drawing;
use crate::model::tree::Tree;

pub struct System {
    pub drawing: Option<Drawing>,
    pub update: bool,
}

impl System {
    pub fn init(&mut self, graphics: &mut dyn Graphics, trees: &Grid<Option<Tree>>) {
        self.drawing = Some(Drawing::init(graphics, trees))
    }

    pub fn update(&mut self) {
        self.update = true;
    }

    pub fn run(
        &mut self,
        graphics: &mut dyn Graphics,
        trees: &Grid<Option<Tree>>,
        terrain: &Grid<f32>,
        piste_map: &Grid<Option<usize>>,
    ) {
        if self.update {
            if let Some(drawing) = &mut self.drawing {
                drawing.update(graphics, trees, terrain, piste_map);
            }
            self.update = false;
        }
    }
}
