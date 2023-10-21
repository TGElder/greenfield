use std::collections::hash_map::Entry;
use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::carousel::draw;
use crate::model::car::Car;
use crate::model::carousel::Carousel;
use crate::model::lift::Lift;

pub fn run(
    carousels: &HashMap<usize, Carousel>,
    lifts: &HashMap<usize, Lift>,
    cars: &HashMap<usize, Car>,
    locations: &HashMap<usize, usize>,
    drawings: &mut HashMap<usize, usize>,
    graphics: &mut dyn Graphics,
) {
    for (id, carousel) in carousels {
        let Some(lift) = lifts.get(id) else {
            continue;
        };
        match drawings.entry(*id) {
            Entry::Occupied(index) => draw(graphics, index.get(), carousel, lift, cars, locations),
            Entry::Vacant(cell) => {
                if let Ok(index) = graphics.create_quads() {
                    draw(graphics, &index, carousel, lift, cars, locations);
                    cell.insert(index);
                }
            }
        }
    }
}
