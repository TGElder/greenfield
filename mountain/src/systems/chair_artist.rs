use std::collections::hash_map::Entry;
use std::collections::HashMap;

use engine::graphics::Graphics;

use crate::draw::chair::draw;
use crate::model::carousel::{Car, Carousel};
use crate::model::lift::Lift;

pub fn run(
    graphics: &mut dyn Graphics,
    carousels: &HashMap<usize, Carousel>,
    lifts: &HashMap<usize, Lift>,
    cars: &HashMap<usize, Car>,
    drawings: &mut HashMap<usize, usize>,
) {
    for carousel in carousels.values() {
        let Some(lift) = lifts.get(&carousel.lift_id) else {
            continue;
        };
        for car_id in carousel.car_ids.iter() {
            let Some(car) = cars.get(car_id) else {
                continue;
            };
            let Some(segment) = lift.segments.get(car.segment) else {
                continue;
            };

            let entry = drawings.entry(*car_id);
            let index = match entry {
                Entry::Occupied(ref value) => value.get(),
                Entry::Vacant(cell) => {
                    let Ok(index) = graphics.create_quads() else {
                        continue;
                    };
                    &*cell.insert(index)
                }
            };

            let vector = segment.to - segment.from;
            let segment_meters = vector.magnitude();
            let p = car.distance_from_start_meters / segment_meters;
            let position = segment.from + vector * p;
            let angle = vector.xy().angle();
            draw(graphics, index, &position, angle);
        }
    }
}
