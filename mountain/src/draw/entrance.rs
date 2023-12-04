use std::iter::once;

use commons::color::Rgb;
use commons::geometry::{xyz, XYRectangle, XYZ};

use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;
use engine::graphics::elements::Quad;
use engine::graphics::models::cube;
use engine::graphics::transform::Transform;
use engine::graphics::Graphics;
use nalgebra::Matrix4;

use crate::model::entrance::Entrance;

const GROUND_TO_BAR_METERS: f32 = 2.0;
const BAR_HEIGHT: f32 = 0.5;
const BLACK: Rgb<f32> = Rgb::new(0.0, 0.0, 0.0);

pub fn draw(
    graphics: &mut dyn Graphics,
    index: &usize,
    Entrance { footprint, .. }: &Entrance,
    terrain: &Grid<f32>,
) {
    let XYRectangle { from, to } = footprint;
    let from = xyz(from.x as f32, from.y as f32, terrain[from]);
    let to = xyz(to.x as f32, to.y as f32, terrain[to]);

    let Some(max_ground_height) = footprint
        .iter()
        .map(|position| terrain[position])
        .max_by(unsafe_ordering)
    else {
        println!("WARN: Cannot draw entrance with no footprint");
        return;
    };
    let bar_bottom_z = max_ground_height + GROUND_TO_BAR_METERS;
    let from_pole_height = bar_bottom_z - from.z;
    let to_pole_height = bar_bottom_z - to.z;

    let from_pole = scaled_and_translated_cube(
        xyz(0.1, 0.1, from_pole_height),
        xyz(from.x, from.y, from.z + from_pole_height / 2.0),
    );
    let to_pole = scaled_and_translated_cube(
        xyz(0.1, 0.1, to_pole_height),
        xyz(to.x, to.y, to.z + to_pole_height / 2.0),
    );
    let banner = scaled_and_translated_cube(
        xyz(
            (from.x - to.x).abs() + 0.1,
            (from.y - to.y).abs() + 0.1,
            BAR_HEIGHT,
        ),
        xyz(
            from.x + (to.x - from.x) / 2.0,
            from.y + (to.y - from.y) / 2.0,
            bar_bottom_z + BAR_HEIGHT / 2.0,
        ),
    );

    let quads = from_pole.chain(to_pole).chain(banner).collect::<Vec<_>>();

    graphics.draw_quads(index, &quads).unwrap();
}

fn scaled_and_translated_cube(
    scale: XYZ<f32>,
    translation: XYZ<f32>,
) -> impl Iterator<Item = Quad> {
    let translation: Matrix4<f32> = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [translation.x, translation.y, translation.z, 1.0],
    ]
    .into();

    let scale: Matrix4<f32> = [
        [scale.x, 0.0, 0.0, 0.0],
        [0.0, scale.y, 0.0, 0.0],
        [0.0, 0.0, scale.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();

    let transformation = translation * scale;

    let cube = cube::model(BLACK);

    once(cube)
        .flatten()
        .map(move |quad| quad.transform(&transformation))
}
