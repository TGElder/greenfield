use commons::color::Rgb;
use commons::grid::Grid;
use engine::graphics::elements::Quad;
use engine::graphics::transform::Transform;
use engine::graphics::utils::triangles_from_quads;
use engine::graphics::Graphics;

use crate::draw::{line, model};
use crate::model::structure::{
    get_wire_path, transformation_matrix_for_structure, Structure, StructureClass,
};

pub fn draw_chain(
    graphics: &mut dyn Graphics,
    index: &usize,
    wire_index: &usize,
    structures: &[&Structure],
    terrain: &Grid<f32>,
) {
    let wire = get_wire_path(structures, terrain);
    line::draw2(graphics, wire_index, &wire, 0.5);

    let quads = structures
        .iter()
        .map(|structure| get_quads(structure, terrain))
        .collect::<Vec<_>>();

    let quads = quads.into_iter().flatten().collect::<Vec<_>>();

    let triangles = triangles_from_quads(&quads);
    graphics.draw_hologram(index, &triangles).unwrap();
}

pub fn get_quads(structure: &Structure, terrain: &Grid<f32>) -> Vec<Quad<Rgb<f32>>> {
    let model = match &structure.class {
        StructureClass::ChairliftBaseStation => model::chairlift::base_station(),
    };
    model
        .quads
        .transform(&transformation_matrix_for_structure(structure, terrain))
}
