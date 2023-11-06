use std::collections::HashMap;

use commons::geometry::{xyz, XYZ};

use crate::model::carousel::{Car, Carousel};
use crate::model::frame::{Frame, Mode};
use crate::model::lift::Lift;

const OFFSET: XYZ<f32> = xyz(0.24, 0.0, -1.4);

pub fn run(
    locations: &HashMap<usize, usize>,
    lifts: &HashMap<usize, Lift>,
    carousels: &HashMap<usize, Carousel>,
    cars: &HashMap<usize, Car>,
    frames: &mut HashMap<usize, Option<Frame>>,
) {
    for (id, location) in locations {
        let Some(car) = cars.get(location) else {
            continue;
        };
        let Some(carousel) = carousels.get(&car.carousel_id) else {
            continue;
        };
        let Some(lift) = lifts.get(&carousel.lift_id) else {
            continue;
        };

        let segment = &lift.segments[car.segment];
        let vector = segment.to - segment.from;
        let segment_meters = vector.magnitude();
        let p = car.distance_from_start_meters / segment_meters;
        let position = segment.from + vector * p;
        let angle = vector.xy().angle();

        frames.insert(
            *id,
            Some(Frame {
                position,
                angle,
                model_offset: Some(OFFSET),
                mode: Mode::Sitting,
            }),
        );
    }
}
