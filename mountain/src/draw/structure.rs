use commons::color::Rgb;
use commons::grid::Grid;
use engine::graphics::elements::Triangle;
use engine::graphics::transform::Transform;
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

    let triangles = structures
        .iter()
        .flat_map(|structure| get_triangles(structure, terrain))
        .collect::<Vec<_>>();

    graphics.draw_hologram(index, &triangles).unwrap();
}

pub fn get_triangles(structure: &Structure, terrain: &Grid<f32>) -> Vec<Triangle<Rgb<f32>>> {
    let triangles = match &structure.class {
        StructureClass::ChairliftPylon => model::chairlift::pylon(),
        StructureClass::ChairliftStation => model::chairlift::station(),
    };
    triangles.transform(&transformation_matrix_for_structure(structure, terrain))
}
