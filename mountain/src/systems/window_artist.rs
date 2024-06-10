use std::collections::HashMap;

use commons::grid::Grid;
use engine::graphics::Graphics;

use crate::draw::windows::Drawing;
use crate::model::building::Building;
use crate::model::door::Door;

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

    pub fn run(
        &mut self,
        terrain: &Grid<f32>,
        buildings: &HashMap<usize, Building>,
        doors: &HashMap<usize, Door>,
        graphics: &mut dyn Graphics,
    ) {
        if !self.update {
            return;
        }
        self.update = false;

        let Some(drawing) = self.drawing.as_ref() else {
            return;
        };

        drawing.draw(graphics, terrain, buildings, &door_to_buildings(doors));
    }
}

fn door_to_buildings(doors: &HashMap<usize, Door>) -> HashMap<usize, Vec<&Door>> {
    let mut out: HashMap<usize, Vec<&Door>> = HashMap::new();
    for door in doors.values() {
        out.entry(door.building_id).or_default().push(door);
    }
    out
}
