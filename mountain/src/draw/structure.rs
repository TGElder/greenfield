use commons::color::Rgb;
use commons::geometry::xyz;
use commons::grid::Grid;
use engine::graphics::elements::Quad;
use engine::graphics::transform::Transform;
use engine::graphics::utils::{transformation_matrix, triangles_from_quads, Transformation};
use engine::graphics::Graphics;

use crate::draw::{line, model};
use crate::model::structure::{Structure, StructureClass};

pub fn draw_chain(
    graphics: &mut dyn Graphics,
    index: &usize,
    wire_index: &usize,
    structures: &[&Structure],
    terrain: &Grid<f32>,
) {
    let quads = structures
        .iter()
        .map(|structure| get_quads(structure, terrain))
        .collect::<Vec<_>>();

    let wire = quads
        .windows(2)
        .flat_map(|window| {
            [
                [window[0][5].corners[0], window[1][5].corners[1]],
                [window[0][5].corners[3], window[1][5].corners[2]],
            ]
        })
        .collect::<Vec<_>>();

    let quads = quads.into_iter().flatten().collect::<Vec<_>>();

    line::draw2(graphics, wire_index, &wire, 0.5);

    let triangles = triangles_from_quads(&quads);
    graphics.draw_hologram(index, &triangles).unwrap();
}

pub fn get_quads(structure: &Structure, terrain: &Grid<f32>) -> Vec<Quad<Rgb<f32>>> {
    let model = match structure.class {
        StructureClass::ChairliftBaseStation => model::chairlift::base_station(),
    };
    model
        .quads
        .transform(&transformation_matrix(Transformation {
            translation: Some(xyz(
                structure.position.x as f32,
                structure.position.y as f32,
                terrain[structure.position],
            )),
            scale: Some(xyz(
                structure.footprint.x as f32,
                structure.footprint.y as f32,
                structure.footprint.z as f32,
            )),
            yaw: Some(structure.rotation),
            ..Transformation::default()
        }))
}
