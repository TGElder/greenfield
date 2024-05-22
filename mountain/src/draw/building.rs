use commons::color::Rgb;
use commons::geometry::{xyz, XYRectangle, XYZ};

use commons::grid::Grid;
use commons::unsafe_ordering::unsafe_ordering;
use engine::graphics::elements::Quad;
use engine::graphics::models::cube;
use engine::graphics::transform::{Recolor, Transform};
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::model::building::Building;

const COLOR: Rgb<f32> = Rgb::new(0.447, 0.361, 0.259);

pub fn draw(graphics: &mut dyn Graphics, index: &usize, building: &Building, terrain: &Grid<f32>) {
    let Building { footprint, height } = building;
    let XYRectangle { from, to } = footprint;
    let from = xyz(from.x as f32, from.y as f32, terrain[from]);
    let to = xyz(to.x as f32, to.y as f32, terrain[to]);

    if footprint.iter().count() == 0 {
        println!("WARN: Cannot draw building with no footprint");
        return;
    }

    let min_ground_height = footprint
        .iter()
        .map(|position| terrain[position])
        .min_by(unsafe_ordering)
        .unwrap();
    let max_ground_height = footprint
        .iter()
        .map(|position| terrain[position])
        .max_by(unsafe_ordering)
        .unwrap();
    let total_height = (max_ground_height - min_ground_height) + (*height as f32);

    let origin = xyz(from.x, from.y, min_ground_height);
    let scale = xyz(to.x - from.x, to.y - from.y, total_height);
    let quads = translated_and_scaled_cube(origin + scale / 2.0, scale, COLOR).collect::<Vec<_>>();

    let triangles = triangles_from_quads(&quads);
    graphics.draw_triangles(index, &triangles).unwrap();
}

fn translated_and_scaled_cube(
    translation: XYZ<f32>,
    scale: XYZ<f32>,
    color: Rgb<f32>,
) -> impl Iterator<Item = Quad<Rgb<f32>>> {
    let transformation = transformation_matrix(Transformation {
        translation: Some(translation),
        scale: Some(scale),
        ..Transformation::default()
    });

    cube::model()
        .transform(&transformation)
        .recolor(&|_| color)
        .into_iter()
}
