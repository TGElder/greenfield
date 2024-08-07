use std::collections::HashMap;

use commons::color::Rgb;

use crate::model::carousel::{Car, Carousel};
use crate::model::frame::{Frame, Model};
use crate::model::lift::Lift;
use crate::model::skier::Clothes;

const MISSING_COLOR: Rgb<f32> = Rgb::new(1.0, 1.0, 0.0);
lazy_static! {
    static ref MISSING_CLOTHES: Clothes<Rgb<f32>> = Clothes {
        skis: MISSING_COLOR,
        trousers: MISSING_COLOR,
        jacket: MISSING_COLOR,
        helmet: MISSING_COLOR,
    };
}

pub fn run(
    carousels: &HashMap<usize, Carousel>,
    lifts: &HashMap<usize, Lift>,
    cars: &HashMap<usize, Car>,
    locations: &HashMap<usize, usize>,
    clothes: &HashMap<usize, Clothes<Rgb<f32>>>,
    frames: &mut HashMap<usize, Option<Frame>>,
) {
    let location_reverse_map = locations
        .iter()
        .map(|(k, v)| (*v, *k))
        .collect::<HashMap<_, _>>();

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

            let vector = segment.to - segment.from;
            let segment_meters = vector.magnitude();
            let p = car.distance_from_start_meters / segment_meters;
            let position = segment.from + vector * p;
            let yaw = vector.xy().angle();

            frames.insert(
                *car_id,
                Some(Frame {
                    position,
                    yaw,
                    pitch: 0.0,
                    model: Model::Chair,
                }),
            );

            // skier riding in chair
            let Some(id) = location_reverse_map.get(car_id) else {
                continue;
            };
            let clothes = clothes.get(id).unwrap_or(&MISSING_CLOTHES);
            frames.insert(
                *id,
                Some(Frame {
                    position,
                    yaw,
                    pitch: 0.0,
                    model: Model::Sitting { clothes: *clothes },
                }),
            );
        }
    }
}
