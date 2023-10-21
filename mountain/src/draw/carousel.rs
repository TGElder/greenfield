use std::collections::HashMap;

use commons::color::Rgb;
use commons::geometry::{xyz, XYZ};
use engine::graphics::elements::Quad;
use engine::graphics::transform::Transform;
use engine::graphics::Graphics;
use nalgebra::Matrix4;

use crate::model::car::Car;
use crate::model::carousel::Carousel;
use crate::model::lift::Lift;

static CAR: Quad = Quad {
    color: Rgb::new(0.0, 1.0, 0.0),
    corners: [
        xyz(-0.25, -0.25, 0.0),
        xyz(0.25, -0.25, 0.0),
        xyz(0.25, 0.25, 0.0),
        xyz(-0.25, 0.25, 0.0),
    ],
};

pub fn draw(
    graphics: &mut dyn Graphics,
    index: &usize,
    carousel: &Carousel,
    lift: &Lift,
    cars: &HashMap<usize, Car>,
    locations: &HashMap<usize, usize>,
) {
    let quads = carousel
        .cars
        .iter()
        .flat_map(|car| {
            let color = if locations.iter().any(|(_, id)| id == car) {
                Rgb::new(1.0, 0.0, 0.0)
            } else {
                Rgb::new(0.0, 1.0, 0.0)
            };

            let mut quad = CAR.transform(&get_translation(&get_position(cars.get(car)?, lift)));
            quad.color = color;

            Some(quad)
        })
        .collect::<Vec<_>>();

    graphics.draw_quads(index, &quads).unwrap();
}

fn get_translation(position: &XYZ<f32>) -> Matrix4<f32> {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [position.x, position.y, position.z, 1.0],
    ]
    .into()
}

fn get_position(car: &Car, lift: &Lift) -> XYZ<f32> {
    let segment = &lift.nodes[car.segment];
    let from = segment.from;
    let to = segment.to;
    let p = car.position_metres / segment.distance_metres;
    xyz(
        from.x * (1.0 - p) + to.x * p,
        from.y * (1.0 - p) + to.y * p,
        from.z * (1.0 - p) + to.z * p,
    )
}
